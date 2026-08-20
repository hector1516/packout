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

fn esc(s: &str) -> String {
    s.replace('\'', "")
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