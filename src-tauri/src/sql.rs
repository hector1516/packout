use crate::config::{AppConfig, SqlDb, Zone};
use futures::StreamExt;
use serde_json::{json, Value};
use std::sync::OnceLock;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

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

fn open(db: &SqlDb, sql: &str) -> Result<Vec<Value>, String> {
    rt().block_on(async {
        let mut client = connect(db).await?;
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
    })
}

fn exec(db: &SqlDb, sql: &str) -> Result<(), String> {
    rt().block_on(async {
        let mut client = connect(db).await?;
        client
            .execute(sql, &[])
            .await
            .map_err(|e| format!("Error ejecutando SQL: {}", e))?;
        Ok(())
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

pub fn guardar_imagen_item(
    cfg: &AppConfig,
    zone: &Zone,
    item: &str,
    imagen: &str,
) -> Result<(), String> {
    let db = cfg.sql_for(Some(zone));
    let sql = format!(
        "IF EXISTS (SELECT 1 FROM {} WHERE Item = '{}') UPDATE {} SET Imagen = '{}' WHERE Item = '{}' ELSE INSERT INTO {} (Item, Imagen) VALUES ('{}', '{}')",
        zone.tables.item_images,
        esc(item),
        zone.tables.item_images,
        esc(imagen),
        esc(item),
        zone.tables.item_images,
        esc(item),
        esc(imagen),
    );
    exec(db, &sql)
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