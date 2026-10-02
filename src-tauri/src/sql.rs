use crate::config::{AppConfig, SqlDb, Zone};
use crate::guard::Guard;
use futures::StreamExt;
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::OnceLock;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

/// Circuito de proteccion contra bloqueo de la cuenta de SQL Server.
pub fn guard() -> &'static Guard {
    static G: OnceLock<Guard> = OnceLock::new();
    G.get_or_init(Guard::default)
}

fn rt() -> &'static Runtime {
    static RT: OnceLock<Runtime> = OnceLock::new();
    RT.get_or_init(|| Runtime::new().expect("crear runtime tokio"))
}

fn config_from(db: &SqlDb) -> Result<Config, String> {
    let cs = format!(
        "Server={};Database={};User Id={};Password={}",
        db.server, db.database, db.user, db.password
    );
    let mut config = Config::from_ado_string(&cs)
        .map_err(|e| format!("Cadena de conexión SQL inválida: {}", e))?;
    config.encryption(tiberius::EncryptionLevel::Required);
    config.trust_cert();
    Ok(config)
}

async fn connect(db: &SqlDb) -> Result<Client<Compat<TcpStream>>, String> {
    let config = config_from(db)?;
    let addr = config.get_addr();
    let timeout = std::time::Duration::from_secs(10);
    let tcp = tokio::time::timeout(timeout, TcpStream::connect(addr.as_str()))
        .await
        .map_err(|_| format!("Timeout conectando a {} (10s)", db.server))?
        .map_err(|e| format!("No se pudo conectar a {}: {}", db.server, e))?;
    let client = tokio::time::timeout(timeout, Client::connect(config, tcp.compat()))
        .await
        .map_err(|_| format!("Timeout de autenticación con {} (10s)", db.server))?
        .map_err(|e| format!("Error de autenticación con {}: {}", db.server, e))?;
    Ok(client)
}

/// Availability check SIN autenticacion: solo abre el socket TCP.
/// Nunca cuenta como intento fallido de login, por eso el polling puede
/// correr cada 20s sin riesgo de bloquear la cuenta.
pub fn ping(server: &str) -> Result<String, String> {
    if server.trim().is_empty() {
        return Err("Servidor vacio".into());
    }
    rt().block_on(async {
        let addr = if server.contains(':') && !server.ends_with("1433") {
            server.to_string()
        } else {
            format!("{}:1433", server)
        };
        let timeout = std::time::Duration::from_secs(4);
        match tokio::time::timeout(timeout, TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => Ok(format!("Servidor {} responde", server)),
            Ok(Err(e)) => Err(format!("Servidor {} no responde: {}", server, e)),
            Err(_) => Err(format!("Servidor {} no responde (timeout 4s)", server)),
        }
    })
}

/// Cierra la sesion explicitamente (envia LOGOUT) para no dejar sockets
/// colgando. Consume el client porque `close()` lo toma por valor.
async fn cerrar(client: Client<Compat<TcpStream>>) {
    let r = tokio::time::timeout(std::time::Duration::from_secs(3), client.close()).await;
    if let Err(e) = r {
        eprintln!("aviso: no se pudo cerrar la conexion SQL: {}", e);
    }
}

/// Toda conexion pasa por aqui: aplica el limite de 3 intentos y bloquea
/// los logins repetidos con contrasena incorrecta.
fn open(db: &SqlDb, sql: &str) -> Result<Vec<Value>, String> {
    guard().check()?;
    let res = open_raw(db, sql);
    guard().record(&res.as_ref().map(|_| ()).map_err(|e| e.clone()));
    res
}

fn open_raw(db: &SqlDb, sql: &str) -> Result<Vec<Value>, String> {
    rt().block_on(async {
        let mut client = match connect(db).await {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let rows = leer(&mut client, sql).await;
        // `stream` ya no vive aqui, el client se puede cerrar.
        cerrar(client).await;
        rows
    })
}

async fn leer(
    client: &mut Client<Compat<TcpStream>>,
    sql: &str,
) -> Result<Vec<Value>, String> {
    let mut stream = client
        .query(sql, &[])
        .await
        .map_err(|e| format!("Error en consulta: {}", e))?;
    let names: Vec<String> = stream
        .columns()
        .await
        .map_err(|e| format!("Error en columnas: {}", e))?
        .unwrap_or_default()
        .iter()
        .map(|c| c.name().to_string())
        .collect();
    let mut rows = Vec::new();
    while let Some(item) = stream.next().await {
        let item = item.map_err(|e| format!("Error leyendo filas: {}", e))?;
        let row = match item {
            tiberius::QueryItem::Row(row) => row,
            _ => continue,
        };
        let mut obj = serde_json::Map::new();
        for (i, name) in names.iter().enumerate() {
            let val: Option<&str> = row.get(i);
            obj.insert(name.clone(), json!(val.unwrap_or_default()));
        }
        rows.push(Value::Object(obj));
    }
    Ok(rows)
}

fn exec(db: &SqlDb, sql: &str) -> Result<(), String> {
    guard().check()?;
    let res = exec_raw(db, sql);
    guard().record(&res);
    res
}

fn exec_raw(db: &SqlDb, sql: &str) -> Result<(), String> {
    rt().block_on(async {
        let mut client = match connect(db).await {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let res = client
            .execute(sql, &[])
            .await
            .map(|_| ())
            .map_err(|e| format!("Error ejecutando SQL: {}", e));
        cerrar(client).await;
        res
    })
}

pub fn test_connection(cfg: &AppConfig, zone: Option<&Zone>) -> Result<String, String> {
    let db = cfg.sql_for(zone);
    let rows = open(db, "SELECT 'OK' AS estado")?;
    let estado = rows
        .first()
        .and_then(|r| r.get("estado"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    Ok(format!(
        "Conectado a SQL Server {} / {} ({})",
        db.server, db.database, estado
    ))
}

pub fn insert_resultado(
    cfg: &AppConfig,
    zone: &Zone,
    fecha_hora: &str,
    pedido: &str,
    serie: &str,
    resultado: &str,
    operador: &str,
    operador_admin: &str,
    comentario: &str,
) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "INSERT INTO {} (FechaHora, Pedido, Serie, Resultado, Operador, OperadorAdmin, Comentario) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}')",
        zone.tables.resultados,
        esc(fecha_hora),
        esc(pedido),
        esc(serie),
        esc(resultado),
        esc(operador),
        esc(operador_admin),
        esc(comentario),
    );
    exec(db, &sql)
}

pub fn consultar_serie_aprobada(
    cfg: &AppConfig,
    zone: &Zone,
    serie: &str,
) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "SELECT TOP 1 * FROM {} WHERE Serie = '{}' AND Resultado = 'APROBADO' ORDER BY FechaHora DESC",
        zone.tables.resultados,
        esc(serie)
    );
    open(db, &sql)
}

pub fn insert_error(
    cfg: &AppConfig,
    zone: &Zone,
    fecha_hora: &str,
    titulo: &str,
    desc: &str,
) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "INSERT INTO {} (FechaHora, Titulo, [Desc]) VALUES ('{}', '{}', '{}')",
        zone.tables.errores,
        esc(fecha_hora),
        esc(titulo),
        esc(desc),
    );
    exec(db, &sql)
}

pub fn consultar_historial(cfg: &AppConfig, zone: &Zone, top: i64) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "SELECT TOP {} * FROM {} ORDER BY FechaHora DESC",
        top, zone.tables.resultados
    );
    open(db, &sql)
}

pub fn consultar_recientes(cfg: &AppConfig, zone: &Zone, top: i64) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "SELECT TOP {} * FROM {} WHERE Resultado = 'APROBADO' ORDER BY FechaHora DESC",
        top, zone.tables.recientes
    );
    open(db, &sql)
}

pub fn consultar_admin(cfg: &AppConfig, zone: &Zone, no: &str) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "SELECT * FROM {} WHERE No = '{}'",
        zone.tables.admin,
        esc(no)
    );
    open(db, &sql)
}

pub fn consultar_operador(cfg: &AppConfig, zone: &Zone, no: &str) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "SELECT * FROM {} WHERE No = '{}'",
        zone.tables.usuarios,
        esc(no)
    );
    open(db, &sql)
}

pub fn obtener_imagen_item(cfg: &AppConfig, zone: &Zone, item: &str) -> Result<Option<String>, String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "SELECT Imagen FROM {} WHERE Item = '{}'",
        zone.tables.item_images,
        esc(item)
    );
    let rows = open(db, &sql)?;
    Ok(rows
        .first()
        .and_then(|r| r.get("Imagen"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

fn upsert_imagen_sql(zone: &Zone, item: &str, imagen: &str) -> String {
    let t = &zone.tables.item_images;
    let ahora = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    format!(
        "IF EXISTS (SELECT 1 FROM {t} WHERE Item = '{item}') UPDATE {t} SET Imagen = '{img}', FechaHora = '{ahora}' WHERE Item = '{item}' ELSE INSERT INTO {t} (Item, Imagen, FechaHora) VALUES ('{item}', '{img}', '{ahora}')",
        t = t,
        item = esc(item),
        img = esc(imagen),
        ahora = esc(&ahora),
    )
}

pub fn guardar_imagen_item(
    cfg: &AppConfig,
    zone: &Zone,
    item: &str,
    imagen: &str,
) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    exec(db, &upsert_imagen_sql(zone, item, imagen))
}

pub fn eliminar_imagen_item(cfg: &AppConfig, zone: &Zone, item: &str) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "DELETE FROM {} WHERE Item = '{}'",
        zone.tables.item_images,
        esc(item)
    );
    exec(db, &sql)
}

pub fn listar_items_con_imagen(cfg: &AppConfig, zone: &Zone) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!("SELECT Item FROM {} ORDER BY Item", zone.tables.item_images);
    open(db, &sql)
}

pub fn expected_columns(zone: &Zone) -> Vec<(String, Vec<(String, String)>, bool, String)> {
    vec![
        (
            zone.tables.resultados.clone(),
            vec![
                ("FechaHora".into(), "VARCHAR(30)".into()),
                ("Pedido".into(), "VARCHAR(50)".into()),
                ("Serie".into(), "VARCHAR(50)".into()),
                ("Resultado".into(), "VARCHAR(30)".into()),
                ("Operador".into(), "VARCHAR(30)".into()),
                ("OperadorAdmin".into(), "VARCHAR(30)".into()),
                ("Comentario".into(), "VARCHAR(500)".into()),
            ],
            false,
            String::new(),
        ),
        (
            zone.tables.errores.clone(),
            vec![
                ("FechaHora".into(), "VARCHAR(30)".into()),
                ("Titulo".into(), "VARCHAR(200)".into()),
                ("Desc".into(), "VARCHAR(1000)".into()),
            ],
            false,
            String::new(),
        ),
        (
            zone.tables.usuarios.clone(),
            vec![
                ("No".into(), "VARCHAR(30)".into()),
                ("Nombre".into(), "VARCHAR(100)".into()),
            ],
            false,
            String::new(),
        ),
        (
            zone.tables.admin.clone(),
            vec![
                ("No".into(), "VARCHAR(30)".into()),
                ("Nombre".into(), "VARCHAR(100)".into()),
            ],
            false,
            String::new(),
        ),
        (
            zone.tables.item_images.clone(),
            vec![
                ("Item".into(), "VARCHAR(50)".into()),
                ("Imagen".into(), "NVARCHAR(MAX)".into()),
                ("FechaHora".into(), "VARCHAR(30)".into()),
            ],
            false,
            String::new(),
        ),
        (
            zone.tables.recientes.clone(),
            vec![
                ("FechaHora".into(), "VARCHAR(30)".into()),
                ("Pedido".into(), "VARCHAR(50)".into()),
                ("Serie".into(), "VARCHAR(50)".into()),
                ("Resultado".into(), "VARCHAR(30)".into()),
                ("Operador".into(), "VARCHAR(30)".into()),
                ("OperadorAdmin".into(), "VARCHAR(30)".into()),
                ("Comentario".into(), "VARCHAR(500)".into()),
            ],
            true,
            format!(
                "CREATE OR ALTER VIEW {} AS SELECT FechaHora, Pedido, Serie, Resultado, Operador, OperadorAdmin, Comentario FROM {}",
                zone.tables.recientes, zone.tables.resultados
            ),
        ),
    ]
}

pub fn table_exists(db: &SqlDb, table: &str) -> Result<bool, String> {
    let rows = open(
        db,
        &format!(
            "SELECT 1 FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '{}'",
            esc(table)
        ),
    )?;
    Ok(!rows.is_empty())
}

pub fn view_exists(db: &SqlDb, view: &str) -> Result<bool, String> {
    let rows = open(
        db,
        &format!(
            "SELECT 1 FROM INFORMATION_SCHEMA.VIEWS WHERE TABLE_NAME = '{}'",
            esc(view)
        ),
    )?;
    Ok(!rows.is_empty())
}

pub fn list_table_columns(db: &SqlDb, table: &str) -> Result<Vec<String>, String> {
    let rows = open(
        db,
        &format!(
            "SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = '{}'",
            esc(table)
        ),
    )?;
    Ok(rows
        .iter()
        .filter_map(|r| r.get("COLUMN_NAME").and_then(|v| v.as_str()))
        .map(|s| s.to_string())
        .collect())
}

pub fn create_table(db: &SqlDb, table: &str, columns: &[(String, String)]) -> Result<(), String> {
    let cols: Vec<String> = columns
        .iter()
        .map(|(name, ty)| format!("[{}] {}", name, ty))
        .collect();
    let sql = format!(
        "IF NOT EXISTS (SELECT 1 FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '{}') CREATE TABLE {} ({})",
        esc(table),
        table,
        cols.join(", ")
    );
    exec(db, &sql)
}

pub fn create_view(db: &SqlDb, source: &str) -> Result<(), String> {
    exec(db, source)
}

pub fn add_missing_columns(
    db: &SqlDb,
    table: &str,
    columns: &[(String, String)],
) -> Result<Vec<String>, String> {
    let existing = list_table_columns(db, table)?;
    let mut added = Vec::new();
    for (name, ty) in columns {
        if !existing.iter().any(|c| c.eq_ignore_ascii_case(name)) {
            let sql = format!("ALTER TABLE {} ADD [{}] {}", table, esc(name), ty);
            exec(db, &sql)?;
            added.push(name.clone());
        }
    }
    Ok(added)
}

pub fn verificar_tablas(cfg: &AppConfig, zone: &Zone) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    let mut out = Vec::new();
    for (table, cols, is_view, _src) in expected_columns(zone) {
        let exists = if is_view {
            view_exists(db, &table)?
        } else {
            table_exists(db, &table)?
        };
        let existing = if exists { list_table_columns(db, &table)? } else { Vec::new() };
        let missing: Vec<String> = cols
            .iter()
            .filter(|(name, _)| !existing.iter().any(|c| c.eq_ignore_ascii_case(name)))
            .map(|(name, _)| name.clone())
            .collect();
        out.push(json!({
            "table": table,
            "exists": exists,
            "isView": is_view,
            "columns": existing,
            "missing": missing,
            "ok": exists && missing.is_empty(),
        }));
    }
    Ok(out)
}

pub fn crear_tablas_faltantes(cfg: &AppConfig, zone: &Zone) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    let mut out = Vec::new();
    for (table, cols, is_view, src) in expected_columns(zone) {
        let exists = if is_view {
            view_exists(db, &table)?
        } else {
            table_exists(db, &table)?
        };
        if !exists {
            if is_view {
                create_view(db, &src)?;
            } else {
                create_table(db, &table, &cols)?;
            }
        } else if is_view {
            let existing = list_table_columns(db, &table)?;
            let missing: Vec<String> = cols
                .iter()
                .filter(|(name, _)| !existing.iter().any(|c| c.eq_ignore_ascii_case(name)))
                .map(|(name, _)| name.clone())
                .collect();
            if !missing.is_empty() {
                create_view(db, &src)?;
            }
        } else {
            add_missing_columns(db, &table, &cols)?;
        }
        let existing = if is_view {
            if exists { list_table_columns(db, &table)? } else { Vec::new() }
        } else {
            list_table_columns(db, &table)?
        };
        let missing: Vec<String> = cols
            .iter()
            .filter(|(name, _)| !existing.iter().any(|c| c.eq_ignore_ascii_case(name)))
            .map(|(name, _)| name.clone())
            .collect();
        out.push(json!({
            "table": table,
            "exists": true,
            "isView": is_view,
            "columns": existing,
            "missing": missing,
            "ok": missing.is_empty(),
        }));
    }
    Ok(out)
}

fn parse_fecha_hora(s: &str) -> Option<chrono::NaiveDateTime> {
    let t = s
        .replace("a. m.", "AM")
        .replace("p. m.", "PM")
        .replace(',', " ")
        .trim()
        .to_string();
    let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
    let fmts = [
        "%d/%m/%Y %H:%M:%S",
        "%d/%m/%Y %I:%M:%S %p",
        "%Y-%m-%d %H:%M:%S",
        "%d-%m-%Y %H:%M:%S",
        "%m/%d/%Y %H:%M:%S",
        "%d/%m/%Y",
        "%Y-%m-%d",
    ];
    for fmt in fmts {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&t, fmt) {
            return Some(dt);
        }
        if let Ok(d) = chrono::NaiveDate::parse_from_str(&t, fmt) {
            return Some(d.and_hms_opt(0, 0, 0).unwrap());
        }
    }
    None
}

pub fn limpiar_registros_antiguos(
    cfg: &AppConfig,
    zone: &Zone,
    dias: i64,
) -> Result<Value, String> {
    let db = cfg.sql_for(Some(zone));
    let cutoff = chrono::Local::now().naive_local() - chrono::Duration::days(dias);
    let mut total_deleted: i64 = 0;
    let mut detalles = Vec::new();
    for table in [&zone.tables.resultados, &zone.tables.errores] {
        if !table_exists(db, table)? {
            detalles.push(json!({"table": table, "deleted": 0, "skipped": true}));
            continue;
        }
        let cols = list_table_columns(db, table)?;
        let has_id = cols.iter().any(|c| c.eq_ignore_ascii_case("Id"));
        let sql = if has_id {
            format!("SELECT Id, FechaHora FROM {}", table)
        } else {
            format!("SELECT FechaHora FROM {}", table)
        };
        let rows = open(db, &sql)?;
        let mut to_delete: Vec<String> = Vec::new();
        let mut ids: Vec<i64> = Vec::new();
        for r in &rows {
            let fh = r.get("FechaHora").and_then(|v| v.as_str()).unwrap_or("");
            if let Some(dt) = parse_fecha_hora(fh) {
                if dt < cutoff {
                    if has_id {
                        if let Some(id) = r.get("Id").and_then(|v| {
                            v.as_i64()
                                .or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok()))
                        }) {
                            ids.push(id);
                        } else {
                            to_delete.push(fh.to_string());
                        }
                    } else {
                        to_delete.push(fh.to_string());
                    }
                }
            }
        }
        let deleted = if has_id && !ids.is_empty() {
            let list = ids.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
            exec(db, &format!("DELETE FROM {} WHERE Id IN ({})", table, list))?;
            ids.len() as i64
        } else if !to_delete.is_empty() {
            for fh in to_delete.iter() {
                exec(
                    db,
                    &format!("DELETE FROM {} WHERE FechaHora = '{}'", table, esc(fh)),
                )?;
            }
            to_delete.len() as i64
        } else {
            0
        };
        total_deleted += deleted;
        detalles.push(json!({"table": table, "deleted": deleted}));
    }
    Ok(json!({"deleted": total_deleted, "cutoff": cutoff.format("%d/%m/%Y").to_string(), "detalles": detalles}))
}

pub fn listar_usuarios(cfg: &AppConfig, zone: &Zone) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    open(db, &format!("SELECT * FROM {} ORDER BY No", zone.tables.usuarios))
}

pub fn insertar_usuario(cfg: &AppConfig, zone: &Zone, no: &str, nombre: &str) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    exec(db, &format!("INSERT INTO {} (No, Nombre) VALUES ('{}', '{}')", zone.tables.usuarios, esc(no), esc(nombre)))
}

pub fn actualizar_usuario(cfg: &AppConfig, zone: &Zone, no: &str, nombre: &str) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    exec(db, &format!("UPDATE {} SET Nombre='{}' WHERE No='{}'", zone.tables.usuarios, esc(nombre), esc(no)))
}

pub fn eliminar_usuario(cfg: &AppConfig, zone: &Zone, no: &str) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    exec(db, &format!("DELETE FROM {} WHERE No='{}'", zone.tables.usuarios, esc(no)))
}

pub fn listar_admins(cfg: &AppConfig, zone: &Zone) -> Result<Vec<Value>, String> {
    let db = cfg.sql_for(Some(zone));
    open(db, &format!("SELECT * FROM {} ORDER BY No", zone.tables.admin))
}

pub fn insertar_admin(cfg: &AppConfig, zone: &Zone, no: &str, nombre: &str) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    exec(db, &format!("INSERT INTO {} (No, Nombre) VALUES ('{}', '{}')", zone.tables.admin, esc(no), esc(nombre)))
}

pub fn actualizar_admin(cfg: &AppConfig, zone: &Zone, no: &str, nombre: &str) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    exec(db, &format!("UPDATE {} SET Nombre='{}' WHERE No='{}'", zone.tables.admin, esc(nombre), esc(no)))
}

pub fn eliminar_admin(cfg: &AppConfig, zone: &Zone, no: &str) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    exec(db, &format!("DELETE FROM {} WHERE No='{}'", zone.tables.admin, esc(no)))
}

pub fn consultar_reportes(
    cfg: &AppConfig,
    zone: &Zone,
    tabla: &str,
    desde: Option<String>,
    hasta: Option<String>,
    resultado: Option<String>,
    busqueda: Option<String>,
    limit: i64,
    offset: i64,
) -> Result<Value, String> {
    let table = match tabla {
        "errores" => &zone.tables.errores,
        _ => &zone.tables.resultados,
    };
    if !table_exists(cfg.sql_for(Some(zone)), table)? {
        return Ok(json!({"rows": [], "total": 0}));
    }
    let rows = open(cfg.sql_for(Some(zone)), &format!("SELECT * FROM {}", table))?;
    let desde_dt = desde.as_deref().and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).map(|d| d.and_hms_opt(0,0,0).unwrap());
    let hasta_dt = hasta.as_deref().and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).map(|d| d.and_hms_opt(23,59,59).unwrap());
    let busqueda_lc = busqueda.as_deref().map(|s| s.to_lowercase()).unwrap_or_default();
    let resultado_lc = resultado.as_deref().map(|s| s.to_lowercase()).unwrap_or_default();
    let mut filtered: Vec<Value> = rows.into_iter().filter(|r| {
        if !resultado_lc.is_empty() {
            let rv = r.get("Resultado").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
            if rv != resultado_lc { return false; }
        }
        if desde_dt.is_some() || hasta_dt.is_some() {
            let fh = r.get("FechaHora").and_then(|v| v.as_str()).unwrap_or("");
            if let Some(dt) = parse_fecha_hora(fh) {
                if let Some(d) = desde_dt { if dt < d { return false; } }
                if let Some(h) = hasta_dt { if dt > h { return false; } }
            } else if desde_dt.is_some() {
                return false;
            }
        }
        if !busqueda_lc.is_empty() {
            let hay = r.as_object().map(|o| o.values().any(|v| {
                let s = if let Some(st) = v.as_str() { st.to_lowercase() } else { v.to_string().to_lowercase() };
                s.contains(&busqueda_lc)
            })).unwrap_or(false);
            if !hay { return false; }
        }
        true
    }).collect();
    filtered.sort_by(|a, b| {
        let da = a.get("FechaHora").and_then(|v| v.as_str()).and_then(parse_fecha_hora);
        let db = b.get("FechaHora").and_then(|v| v.as_str()).and_then(parse_fecha_hora);
        db.cmp(&da)
    });
    let total = filtered.len() as i64;
    let paged: Vec<Value> = filtered.into_iter().skip(offset as usize).take(limit as usize).collect();
    Ok(json!({"rows": paged, "total": total}))
}

fn esc(s: &str) -> String {
    s.replace('\'', "")
}

/// Extrae los 3 primeros octetos desde "10.96.16.114", "10.96.16" o "10.96.16.0/24".
fn parse_subnet24(base: &str) -> Result<(u8, u8, u8), String> {
    let b = base
        .trim()
        .trim_end_matches("/24")
        .trim_end_matches(".0")
        .trim_end_matches('.');
    let parts: Vec<&str> = b.split('.').collect();
    let oct = |i: usize| {
        parts
            .get(i)
            .ok_or_else(|| "IP incompleta".to_string())
            .and_then(|p| {
                p.parse::<u8>()
                    .map_err(|_| format!("Octeto inválido: '{}'", p))
            })
    };
    match parts.len() {
        3 => Ok((oct(0)?, oct(1)?, oct(2)?)),
        4 => Ok((oct(0)?, oct(1)?, oct(2)?)),
        _ => Err(
            "Escribe la IP base, ej. 10.96.16.114 o 10.96.16.0/24".to_string(),
        ),
    }
}

/// Rastrea la red /24 de la IP dada probando TCP al puerto 1433 (SQL Server).
/// Devuelve las IPs que aceptan conexión.
pub fn scan_subnet(base: &str) -> Result<Vec<String>, String> {
    let (a, b, c) = parse_subnet24(base)?;
    rt().block_on(async move {
        let timeout = std::time::Duration::from_millis(500);
        let probes = (1u8..=254).map(|h| async move {
            let ip = format!("{}.{}.{}.{}", a, b, c, h);
            let addr = format!("{}:1433", ip);
            match tokio::time::timeout(timeout, TcpStream::connect(&addr)).await {
                Ok(Ok(_)) => Some(ip),
                _ => None,
            }
        });
        let results = futures::future::join_all(probes).await;
        let mut found: Vec<String> = results.into_iter().flatten().collect();
        found.sort();
        Ok(found)
    })
}

/// Lista las bases de datos del servidor usando el usuario y password dados.
/// Se conecta a `master` y lee `sys.databases`.
pub fn list_databases(server: &str, user: &str, password: &str) -> Result<Vec<String>, String> {
    if server.trim().is_empty() {
        return Err("Indica el servidor primero".to_string());
    }
    let db = SqlDb {
        server: server.trim().to_string(),
        database: "master".to_string(),
        user: user.to_string(),
        password: password.to_string(),
        driver: "SQL Server".to_string(),
    };
    let rows = open(&db, "SELECT name FROM sys.databases ORDER BY name")?;
    Ok(rows
        .iter()
        .filter_map(|r| {
            r.get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Importacion masiva de imagenes desde una carpeta
// ---------------------------------------------------------------------------

const EXTENSIONES_IMAGEN: [&str; 8] = ["png", "jpg", "jpeg", "webp", "gif", "bmp", "tif", "tiff"];

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportProgress {
    pub processed: usize,
    pub total: usize,
    pub saved: usize,
    pub failed: usize,
    pub skipped: usize,
    pub current: String,
    pub pct: f64,
    pub elapsed_ms: u64,
    pub eta_ms: Option<u64>,
    pub speed_per_sec: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportSummary {
    pub total: usize,
    pub saved: usize,
    pub failed: usize,
    pub skipped: usize,
    pub elapsed_ms: u64,
    pub errores: Vec<String>,
}

/// Codificador base64 estandar, sin dependencia extra.
fn base64_encode(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(T[((n >> 18) & 63) as usize] as char);
        out.push(T[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            T[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            T[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn mime_de(xtension: &str) -> &'static str {
    match xtension {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "tif" | "tiff" => "image/tiff",
        _ => "application/octet-stream",
    }
}

/// Lista los archivos de imagen de una carpeta (con subcarpetas si se pide).
fn listar_archivos_imagen(dir: &std::path::Path, recursivo: bool) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    let mut dirs = Vec::new();
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            if recursivo {
                dirs.push(p);
            }
            continue;
        }
        let ext = p
            .extension()
            .map(|x| x.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        if EXTENSIONES_IMAGEN.contains(&ext.as_str()) {
            out.push(p);
        }
    }
    if recursivo {
        for d in dirs {
            out.extend(listar_archivos_imagen(&d, recursivo));
        }
    }
    out.sort();
    out
}

/// Importa todas las imagenes de una carpeta.
///
/// El nombre del archivo (sin extension) es el codigo del item.
/// Usa UNA sola conexion para todo el lote: 500 imagenes = 1 login, no 500.
pub fn importar_imagenes_carpeta<F>(
    cfg: &AppConfig,
    zone: &Zone,
    carpeta: &str,
    recursivo: bool,
    overwrite: bool,
    mut on_progress: F,
) -> Result<ImportSummary, String>
where
    F: FnMut(&ImportProgress),
{
    let dir = std::path::PathBuf::from(carpeta);
    if !dir.is_dir() {
        return Err(format!("No es una carpeta: {}", carpeta));
    }
    let archivos = listar_archivos_imagen(&dir, recursivo);
    let total = archivos.len();
    let inicio = std::time::Instant::now();

    if total == 0 {
        return Ok(ImportSummary {
            total: 0,
            saved: 0,
            failed: 0,
            skipped: 0,
            elapsed_ms: 0,
            errores: vec![],
        });
    }

    // Items que ya existen, para saltar si overwrite = false.
    let existentes: Vec<String> = if overwrite {
        Vec::new()
    } else {
        listar_items_con_imagen(cfg, zone)
            .unwrap_or_default()
            .iter()
            .filter_map(|r| r.get("Item").and_then(|v| v.as_str()).map(|s| s.to_string()))
            .collect()
    };

    guard().check()?;
    let db = cfg.sql_for(Some(zone));

    let resultado = rt().block_on(async {
        let mut client = match connect(db).await {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let mut saved = 0usize;
        let mut failed = 0usize;
        let mut skipped = 0usize;
        let mut errores: Vec<String> = Vec::new();

        for (i, path) in archivos.iter().enumerate() {
            let item = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
                .trim()
                .to_uppercase();
            let nombre = path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();

            if item.is_empty() {
                failed += 1;
                errores.push(format!("{}: nombre de archivo vacio", nombre));
                continue;
            }
            if existentes.iter().any(|e| e.eq_ignore_ascii_case(&item)) {
                skipped += 1;
                continue;
            }

            let ext = path
                .extension()
                .map(|x| x.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let bytes = match std::fs::read(path) {
                Ok(b) => b,
                Err(e) => {
                    failed += 1;
                    if errores.len() < 50 {
                        errores.push(format!("{}: {}", nombre, e));
                    }
                    continue;
                }
            };
            let data_url = format!(
                "data:{};base64,{}",
                mime_de(&ext),
                base64_encode(&bytes)
            );
            let sql = upsert_imagen_sql(zone, &item, &data_url);
            match client.execute(&sql, &[]).await {
                Ok(_) => saved += 1,
                Err(e) => {
                    let msg = format!("Error en {}: {}", nombre, e);
                    if crate::guard::is_auth_error(&msg) {
                        // Falla de credenciales: aborta de inmediato, no seguimos
                        // acumulando intentos que bloqueen la cuenta.
                        return Err(msg);
                    }
                    failed += 1;
                    if errores.len() < 50 {
                        errores.push(msg);
                    }
                }
            }

            let processed = i + 1;
            let elapsed_ms = inicio.elapsed().as_millis() as u64;
            let speed = processed as f64 / (elapsed_ms.max(1) as f64 / 1000.0);
            let eta_ms = if speed > 0.0 {
                Some((((total - processed) as f64) / speed * 1000.0) as u64)
            } else {
                None
            };
            on_progress(&ImportProgress {
                processed,
                total,
                saved,
                failed,
                skipped,
                current: nombre,
                pct: (processed as f64 / total as f64) * 100.0,
                elapsed_ms,
                eta_ms,
                speed_per_sec: speed,
            });
        }

        cerrar(client).await;
        Ok(ImportSummary {
            total,
            saved,
            failed,
            skipped,
            elapsed_ms: inicio.elapsed().as_millis() as u64,
            errores,
        })
    });

    match resultado {
        Ok(s) => {
            guard().record(&Ok(()));
            Ok(s)
        }
        Err(e) => {
            guard().record(&Err(e.clone()));
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    fn local_cfg() -> AppConfig {
        let appdata = std::env::var("APPDATA").expect("APPDATA");
        let dir = std::path::PathBuf::from(appdata).join("com.packout.app");
        config::load(&dir).expect("cargar config")
    }

    #[test]
    fn conecta_a_localdb() {
        let cfg = local_cfg();
        let zone = cfg.active().expect("zona activa");
        let msg = test_connection(&cfg, Some(zone)).unwrap_or_else(|e| panic!("conexión: {}", e));
        assert!(msg.contains("Conectado"));
    }

    #[test]
    fn consulta_historial_ok() {
        let cfg = local_cfg();
        let zone = cfg.active().expect("zona activa");
        let rows = consultar_historial(&cfg, zone, 10).expect("historial");
        assert!(!rows.is_empty());
    }

    #[test]
    fn login_admin_ok() {
        let cfg = local_cfg();
        let zone = cfg.active().expect("zona activa");
        let rows = consultar_admin(&cfg, zone, "9001").expect("admin");
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn login_operador_ok() {
        let cfg = local_cfg();
        let zone = cfg.active().expect("zona activa");
        let rows = consultar_operador(&cfg, zone, "1001").expect("operador");
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn inserta_y_recientes_ok() {
        let cfg = local_cfg();
        let zone = cfg.active().expect("zona activa");
        insert_resultado(&cfg, zone, "2026/08/10 12:00:00", "P8000", "MY8000", "APROBADO", "1001", "9001", "test insert").expect("insert");
        let rows = consultar_recientes(&cfg, zone, 5).expect("recientes");
        assert!(rows.iter().any(|r| r["Serie"] == "MY8000"));
    }

    #[test]
    fn imagen_item_save_get_ok() {
        let cfg = local_cfg();
        let zone = cfg.active().expect("zona activa");
        let fake = "FAKEIMG";
        guardar_imagen_item(&cfg, zone, fake, "data:image/png;base64,AAAA").expect("save");
        let got = obtener_imagen_item(&cfg, zone, fake).expect("get").expect("alguna");
        assert_eq!(got, "data:image/png;base64,AAAA");
        let list = listar_items_con_imagen(&cfg, zone).expect("list");
        assert!(list.iter().any(|r| r["Item"] == fake));
        eliminar_imagen_item(&cfg, zone, fake).expect("delete");
        assert!(obtener_imagen_item(&cfg, zone, fake).expect("get2").is_none());
    }
}