use crate::config::{AppConfig, SqlDb, Zone};
use crate::mapics::odbc_err;
use odbc::{create_environment_v3, DiagnosticRecord, ResultSetState, Statement};
use serde_json::{json, Value};

fn odbc_env_err(e: Option<DiagnosticRecord>) -> String {
    e.map(|d| d.to_string())
        .unwrap_or_else(|| "Error ODBC (sin detalles)".into())
}

fn conn_string(db: &SqlDb) -> String {
    format!(
        "Driver={{{}}};Server={};Database={};Uid={};Pwd={};MultipleActiveResultSets=False",
        db.driver, db.server, db.database, db.user, db.password
    )
}

fn open(db: &SqlDb, sql: &str) -> Result<Vec<Value>, String> {
    let env = create_environment_v3().map_err(odbc_env_err)?;
    let conn = env
        .connect_with_connection_string(&conn_string(db))
        .map_err(odbc_err)?;
    let stmt = Statement::with_parent(&conn).map_err(odbc_err)?;
    let state = stmt.exec_direct(sql).map_err(odbc_err)?;
    match state {
        ResultSetState::NoData(_) => Ok(Vec::new()),
        ResultSetState::Data(mut stmt) => {
            let ncols = stmt.num_result_cols().map_err(odbc_err)? as usize;
            let labels: Vec<String> = (1..=ncols as u16)
                .map(|i| {
                    stmt.describe_col(i)
                        .map(|d| d.name)
                        .unwrap_or_else(|_| format!("col{}", i))
                })
                .collect();
            let mut rows = Vec::new();
            while let Some(mut cursor) = stmt.fetch().map_err(odbc_err)? {
                let mut obj = serde_json::Map::new();
                for (idx, label) in labels.iter().enumerate() {
                    let col = (idx + 1) as u16;
                    let val: String = cursor
                        .get_data::<String>(col)
                        .map_err(odbc_err)?
                        .unwrap_or_default();
                    obj.insert(label.clone(), json!(val));
                }
                rows.push(Value::Object(obj));
            }
            Ok(rows)
        }
    }
}

fn exec(db: &SqlDb, sql: &str) -> Result<(), String> {
    let env = create_environment_v3().map_err(odbc_env_err)?;
    let conn = env
        .connect_with_connection_string(&conn_string(db))
        .map_err(odbc_err)?;
    let stmt = Statement::with_parent(&conn).map_err(odbc_err)?;
    stmt.exec_direct(sql).map_err(odbc_err)?;
    Ok(())
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