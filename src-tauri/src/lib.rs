mod config;
mod mapics;
mod sql;
#[cfg(desktop)]
mod updater;

use config::AppConfig;
use serde_json::json;
use tauri::Manager;

struct AppState {
    app_data_dir: std::path::PathBuf,
}

fn load_config(state: &AppState) -> Result<AppConfig, String> {
    config::load(&state.app_data_dir)
}

#[tauri::command]
fn get_config(state: tauri::State<AppState>) -> Result<AppConfig, String> {
    load_config(&state)
}

#[tauri::command]
fn save_config(
    state: tauri::State<AppState>,
    config: AppConfig,
) -> Result<(), String> {
    config::save(&state.app_data_dir, &config)
}

#[tauri::command]
fn export_config(
    state: tauri::State<AppState>,
    path: String,
) -> Result<(), String> {
    let cfg = load_config(&state)?;
    config::export_to(&std::path::Path::new(&path), &cfg)
}

#[tauri::command]
fn import_config(
    state: tauri::State<AppState>,
    path: String,
) -> Result<AppConfig, String> {
    let cfg = config::import_from(&std::path::Path::new(&path))?;
    config::save(&state.app_data_dir, &cfg)?;
    Ok(cfg)
}

#[tauri::command]
fn set_active_zone(
    state: tauri::State<AppState>,
    zone_id: String,
) -> Result<AppConfig, String> {
    let mut cfg = load_config(&state)?;
    if !cfg.zones.iter().any(|z| z.id == zone_id) {
        return Err(format!("Zona '{}' no encontrada", zone_id));
    }
    cfg.active_zone = zone_id;
    config::save(&state.app_data_dir, &cfg)?;
    Ok(cfg)
}

#[tauri::command]
fn test_zone(state: tauri::State<AppState>) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();

    let sql_result = sql::test_connection(&cfg, Some(&zone));
    let mapics_result = mapics::test_connection(&zone);

    Ok(json!({
        "zone": zone.id,
        "sql": match sql_result { Ok(s) => json!({"ok": true, "msg": s}), Err(e) => json!({"ok": false, "msg": e}) },
        "mapics": match mapics_result { Ok(s) => json!({"ok": true, "msg": s}), Err(e) => json!({"ok": false, "msg": e}) },
    }))
}

#[tauri::command]
fn mapics_query_kit(
    state: tauri::State<AppState>,
    serie: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = mapics::query_kit(&zone, &serie)?;
    Ok(json!({ "rows": rows, "count": rows.len() }))
}

#[tauri::command]
fn mapics_insert_kit(
    state: tauri::State<AppState>,
    serie: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let affected = mapics::insert_kit(&zone, &serie)?;
    Ok(json!({ "affected": affected }))
}

#[tauri::command]
fn mapics_delete_kit(
    state: tauri::State<AppState>,
    serie: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let affected = mapics::delete_kit(&zone, &serie)?;
    Ok(json!({ "affected": affected }))
}

#[tauri::command]
fn sql_historial(
    state: tauri::State<AppState>,
    top: Option<i64>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let top = top.unwrap_or(10);
    let rows = sql::consultar_historial(&cfg, &zone, top)?;
    Ok(json!({ "rows": rows, "count": rows.len() }))
}

#[tauri::command]
fn sql_item_image(
    state: tauri::State<AppState>,
    item: String,
) -> Result<Option<String>, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    sql::obtener_imagen_item(&cfg, &zone, &item)
}

#[tauri::command]
fn sql_save_item_image(
    state: tauri::State<AppState>,
    item: String,
    imagen: String,
) -> Result<(), String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    sql::guardar_imagen_item(&cfg, &zone, &item, &imagen)
}

#[tauri::command]
fn sql_delete_item_image(
    state: tauri::State<AppState>,
    item: String,
) -> Result<(), String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    sql::eliminar_imagen_item(&cfg, &zone, &item)
}

#[tauri::command]
fn sql_list_item_images(state: tauri::State<AppState>) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = sql::listar_items_con_imagen(&cfg, &zone)?;
    Ok(json!({ "rows": rows, "count": rows.len() }))
}

fn write_local_log(app_data_dir: &std::path::Path, msg: &str) {
    let log = app_data_dir.join("packout.log");
    let line = format!(
        "{} {}\r\n",
        chrono::Local::now().format("%Y/%m/%d %H:%M:%S"),
        msg
    );
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(log) {
        let _ = f.write_all(line.as_bytes());
    }
}

#[tauri::command]
fn sql_insert_error(
    state: tauri::State<AppState>,
    titulo: String,
    desc: String,
) -> Result<(), String> {
    let cfg = load_config(&state)?;
    write_local_log(&state.app_data_dir, format!("[{}] {}", titulo, desc).as_str());
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    sql::insert_error(
        &cfg,
        &zone,
        &chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string(),
        &titulo,
        &desc,
    )
}

#[tauri::command]
fn sql_recientes(
    state: tauri::State<AppState>,
    top: Option<i64>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let top = top.unwrap_or(20);
    let rows = sql::consultar_recientes(&cfg, &zone, top)?;
    Ok(json!({ "rows": rows, "count": rows.len() }))
}

#[tauri::command]
fn sql_login(
    state: tauri::State<AppState>,
    no: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = sql::consultar_admin(&cfg, &zone, &no)?;
    Ok(json!({ "rows": rows, "found": !rows.is_empty() }))
}

#[tauri::command]
fn sql_check_operator(
    state: tauri::State<AppState>,
    no: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = sql::consultar_operador(&cfg, &zone, &no)?;
    Ok(json!({ "rows": rows, "found": !rows.is_empty() }))
}

#[tauri::command]
fn sql_insert_resultado(
    state: tauri::State<AppState>,
    pedido: String,
    serie: String,
    resultado: String,
    operador: String,
    operador_admin: String,
    comentario: String,
) -> Result<(), String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    sql::insert_resultado(
        &cfg,
        &zone,
        &chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string(),
        &pedido,
        &serie,
        &resultado,
        &operador,
        &operador_admin,
        &comentario,
    )
}

#[tauri::command]
fn reimprimir(
    state: tauri::State<AppState>,
    serie: String,
    operador_admin: String,
) -> Result<String, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();

    let del = mapics::delete_kit(&zone, &serie)?;
    let ins = mapics::insert_kit(&zone, &serie)?;
    let fecha = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    sql::insert_resultado(
        &cfg,
        &zone,
        &fecha,
        "",
        &serie,
        "REIMPRESO",
        "N/A",
        &operador_admin,
        &format!("Serie reimpresa: {}", serie),
    )?;
    sql::insert_error(
        &cfg,
        &zone,
        &fecha,
        "Reimpresión",
        &format!("Numero de serie reimpreso: {}", serie),
    )?;
    Ok(json!({ "deleted": del, "inserted": ins }).to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_config_dir()
                .map_err(|e| format!("No se pudo obtener config dir: {}", e))?;
            let _ = config::load(&app_data_dir);
            let host = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "desconocido".into());
            write_local_log(&app_data_dir, &format!("inicio de la app en: PC {}", host));
            app.manage(AppState { app_data_dir });
            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;
                app.manage(updater::PendingUpdate(std::sync::Mutex::new(None)));
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            export_config,
            import_config,
            set_active_zone,
            test_zone,
            mapics_query_kit,
            mapics_insert_kit,
            mapics_delete_kit,
            sql_historial,
            sql_recientes,
            sql_insert_resultado,
            sql_insert_error,
            sql_login,
            sql_check_operator,
            reimprimir,
            sql_item_image,
            sql_save_item_image,
            sql_delete_item_image,
            sql_list_item_images,
            #[cfg(desktop)]
            updater::check_update,
            #[cfg(desktop)]
            updater::install_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}