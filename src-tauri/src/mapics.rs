use crate::config::{render, Zone};
use odbc::{create_environment_v3, ResultSetState, Statement};
use serde_json::{json, Value};

pub fn odbc_err(e: odbc::DiagnosticRecord) -> String {
    e.to_string()
}

fn odbc_env_err(e: Option<odbc::DiagnosticRecord>) -> String {
    e.map(|d| d.to_string())
        .unwrap_or_else(|| "Error ODBC (sin detalles)".into())
}

fn conn_string(zone: &Zone) -> String {
    format!(
        "DSN={};UID={};PWD={}",
        zone.mapics.dsn, zone.mapics.user, zone.mapics.password
    )
}

pub const TEST_QUERY: &str = "SELECT 1 FROM SYSIBM.SYSDUMMY1";

/// Ejecuta un SELECT y devuelve (columnas, filas).
pub fn run_select(
    cs: &str,
    sql: &str,
) -> Result<(Vec<String>, Vec<std::collections::BTreeMap<String, Value>>), String> {
    let env = create_environment_v3().map_err(odbc_env_err)?;
    let conn = env.connect_with_connection_string(cs).map_err(odbc_err)?;
    let stmt = Statement::with_parent(&conn).map_err(odbc_err)?;
    let state = stmt.exec_direct(sql).map_err(odbc_err)?;
    match state {
        ResultSetState::NoData(_) => Ok((Vec::new(), Vec::new())),
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
                let mut obj = std::collections::BTreeMap::new();
                for (idx, label) in labels.iter().enumerate() {
                    let col = (idx + 1) as u16;
                    let val: String = cursor
                        .get_data::<String>(col)
                        .map_err(odbc_err)?
                        .unwrap_or_default();
                    obj.insert(label.clone(), json!(val));
                }
                rows.push(obj);
            }
            Ok((labels, rows))
        }
    }
}

pub struct TestResult {
    pub columns: Vec<String>,
    pub rows: Vec<std::collections::BTreeMap<String, Value>>,
}

pub fn test_connection_params(
    dsn: &str,
    user: &str,
    password: &str,
    server: &str,
) -> Result<TestResult, String> {
    if dsn.trim().is_empty() {
        return Err("Falta el DSN de MAPICS".into());
    }
    let cs = format!("DSN={};UID={};PWD={}", dsn, user, password);
    let (columns, rows) = run_select(&cs, TEST_QUERY)?;
    let _ = server;
    Ok(TestResult { columns, rows })
}

pub fn test_connection(zone: &Zone) -> Result<String, String> {
    let r = test_connection_params(
        &zone.mapics.dsn,
        &zone.mapics.user,
        &zone.mapics.password,
        &zone.mapics.server,
    )?;
    Ok(format!(
        "Conectado a MAPICS {} (servidor {}) — {} fila(s) de '{}'",
        zone.mapics.dsn,
        zone.mapics.server,
        r.rows.len(),
        TEST_QUERY
    ))
}

pub fn query_kit(zone: &Zone, serie: &str) -> Result<Vec<Value>, String> {
    let sql = render(&zone.mapics.query_kit, serie, &zone.estacion);
    let env = create_environment_v3().map_err(odbc_env_err)?;
    let conn = env
        .connect_with_connection_string(&conn_string(zone))
        .map_err(odbc_err)?;
    let stmt = Statement::with_parent(&conn).map_err(odbc_err)?;
    let state = stmt.exec_direct(&sql).map_err(odbc_err)?;
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

fn execute_no_result(zone: &Zone, sql: &str) -> Result<usize, String> {
    let env = create_environment_v3().map_err(odbc_env_err)?;
    let conn = env
        .connect_with_connection_string(&conn_string(zone))
        .map_err(odbc_err)?;
    let stmt = Statement::with_parent(&conn).map_err(odbc_err)?;
    let state = stmt.exec_direct(sql).map_err(odbc_err)?;
    match state {
        ResultSetState::NoData(stmt) => Ok(stmt.affected_row_count().map_err(odbc_err)? as usize),
        ResultSetState::Data(stmt) => Ok(stmt.affected_row_count().map_err(odbc_err)? as usize),
    }
}

pub fn insert_kit(zone: &Zone, serie: &str) -> Result<usize, String> {
    let sql = render(&zone.mapics.query_insert, serie, &zone.estacion);
    execute_no_result(zone, &sql)
}

pub fn delete_kit(zone: &Zone, serie: &str) -> Result<usize, String> {
    let sql = render(&zone.mapics.query_delete, serie, &zone.estacion);
    execute_no_result(zone, &sql)
}

pub fn query_buffer_serials(zone: &Zone, serie: &str, limit: u32) -> Result<Vec<String>, String> {
    let sql = zone
        .mapics
        .query_buffer
        .replace("{SERIE}", serie)
        .replace("{ESTACION}", &zone.estacion)
        .replace("{LIMIT}", &limit.to_string());
    let env = create_environment_v3().map_err(odbc_env_err)?;
    let conn = env
        .connect_with_connection_string(&conn_string(zone))
        .map_err(odbc_err)?;
    let stmt = Statement::with_parent(&conn).map_err(odbc_err)?;
    let state = stmt.exec_direct(&sql).map_err(odbc_err)?;
    match state {
        ResultSetState::NoData(_) => Ok(Vec::new()),
        ResultSetState::Data(mut stmt) => {
            let mut serials = Vec::new();
            while let Some(mut cursor) = stmt.fetch().map_err(odbc_err)? {
                let val: String = cursor
                    .get_data::<String>(1)
                    .map_err(odbc_err)?
                    .unwrap_or_default();
                if !val.is_empty() {
                    serials.push(val.trim().to_string());
                }
            }
            Ok(serials)
        }
    }
}