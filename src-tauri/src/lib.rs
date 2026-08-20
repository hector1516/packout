mod cache;
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

async fn run_blocking<T, F>(f: F) -> Result<T, String>
where
    F: FnOnce() -> Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| format!("Tarea en segundo plano falló: {}", e))?
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
async fn test_zone(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();

    let (sql_result, mapics_result) = tokio::join!(
        run_blocking({
            let cfg = cfg.clone();
            let zone = zone.clone();
            move || sql::test_connection(&cfg, Some(&zone))
        }),
        run_blocking({
            let zone = zone.clone();
            move || mapics::test_connection(&zone)
        }),
    );

    Ok(json!({
        "zone": zone.id,
        "sql": match sql_result { Ok(s) => json!({"ok": true, "msg": s}), Err(e) => json!({"ok": false, "msg": e}) },
        "mapics": match mapics_result { Ok(s) => json!({"ok": true, "msg": s}), Err(e) => json!({"ok": false, "msg": e}) },
    }))
}

#[tauri::command]
async fn mapics_query_kit(
    state: tauri::State<'_, AppState>,
    serie: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = run_blocking(move || mapics::query_kit(&zone, &serie)).await?;
    Ok(json!({ "rows": rows, "count": rows.len() }))
}

#[tauri::command]
async fn mapics_insert_kit(
    state: tauri::State<'_, AppState>,
    serie: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let affected = run_blocking(move || mapics::insert_kit(&zone, &serie)).await?;
    Ok(json!({ "affected": affected }))
}

#[tauri::command]
async fn mapics_delete_kit(
    state: tauri::State<'_, AppState>,
    serie: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let affected = run_blocking(move || mapics::delete_kit(&zone, &serie)).await?;
    Ok(json!({ "affected": affected }))
}

#[tauri::command]
async fn sql_historial(
    state: tauri::State<'_, AppState>,
    top: Option<i64>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let top = top.unwrap_or(10);
    let rows = run_blocking(move || sql::consultar_historial(&cfg, &zone, top)).await?;
    Ok(json!({ "rows": rows, "count": rows.len() }))
}

#[tauri::command]
async fn sql_item_image(
    state: tauri::State<'_, AppState>,
    item: String,
) -> Result<Option<String>, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    run_blocking(move || sql::obtener_imagen_item(&cfg, &zone, &item)).await
}

#[tauri::command]
async fn sql_save_item_image(
    state: tauri::State<'_, AppState>,
    item: String,
    imagen: String,
) -> Result<(), String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    run_blocking(move || sql::guardar_imagen_item(&cfg, &zone, &item, &imagen)).await
}

#[tauri::command]
async fn sql_delete_item_image(
    state: tauri::State<'_, AppState>,
    item: String,
) -> Result<(), String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    run_blocking(move || sql::eliminar_imagen_item(&cfg, &zone, &item)).await
}

#[tauri::command]
async fn sql_list_item_images(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = run_blocking(move || sql::listar_items_con_imagen(&cfg, &zone)).await?;
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
async fn sql_insert_error(
    state: tauri::State<'_, AppState>,
    titulo: String,
    desc: String,
) -> Result<(), String> {
    let cfg = load_config(&state)?;
    write_local_log(&state.app_data_dir, format!("[{}] {}", titulo, desc).as_str());
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let fecha = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    run_blocking(move || sql::insert_error(&cfg, &zone, &fecha, &titulo, &desc)).await
}

#[tauri::command]
async fn sql_recientes(
    state: tauri::State<'_, AppState>,
    top: Option<i64>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let top = top.unwrap_or(20);
    let rows = run_blocking(move || sql::consultar_recientes(&cfg, &zone, top)).await?;
    Ok(json!({ "rows": rows, "count": rows.len() }))
}

#[tauri::command]
async fn sql_login(
    state: tauri::State<'_, AppState>,
    no: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = run_blocking(move || sql::consultar_admin(&cfg, &zone, &no)).await?;
    Ok(json!({ "rows": rows, "found": !rows.is_empty() }))
}

#[tauri::command]
async fn sql_check_operator(
    state: tauri::State<'_, AppState>,
    no: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = run_blocking(move || sql::consultar_operador(&cfg, &zone, &no)).await?;
    Ok(json!({ "rows": rows, "found": !rows.is_empty() }))
}

#[tauri::command]
async fn sql_check_serie_aprobada(
    state: tauri::State<'_, AppState>,
    serie: String,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = run_blocking(move || sql::consultar_serie_aprobada(&cfg, &zone, &serie)).await?;
    Ok(json!({ "rows": rows, "found": !rows.is_empty() }))
}

#[tauri::command]
async fn mapics_precache(
    state: tauri::State<'_, AppState>,
    serie: String,
    limit: Option<u32>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let n = limit.unwrap_or(cfg.buffer_kits);
    let dir = state.app_data_dir.clone();
    let added =
        run_blocking(move || cache::precache(&dir, &zone, &serie, n)).await?;
    Ok(json!({ "added": added }))
}

#[tauri::command]
fn cache_snapshot(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let cache = cache::load(&state.app_data_dir);
    Ok(json!({
        "kits": cache.kits.iter().map(|k| json!({
            "serie": k.serie,
            "pedido": k.pedido,
            "items": k.items.len(),
            "image": k.image.is_some(),
        })).collect::<Vec<_>>(),
        "cola": cache.cola,
        "procesadas": cache.procesadas,
    }))
}

#[tauri::command]
fn cache_get_kit(
    state: tauri::State<'_, AppState>,
    serie: String,
) -> Result<serde_json::Value, String> {
    let kit = cache::get_kit(&state.app_data_dir, &serie);
    match kit {
        Some(k) => Ok(json!({
            "found": true,
            "serie": k.serie,
            "pedido": k.pedido,
            "items": k.items,
            "image": k.image,
        })),
        None => Ok(json!({ "found": false })),
    }
}

#[tauri::command]
fn cache_save_kit(
    state: tauri::State<'_, AppState>,
    serie: String,
    pedido: String,
    items: Vec<cache::CachedItem>,
    image: Option<String>,
) -> Result<(), String> {
    cache::set_kit(
        &state.app_data_dir,
        cache::CachedKit {
            serie,
            pedido,
            items,
            image,
        },
    )
}

#[tauri::command]
fn cache_get_foto(
    state: tauri::State<'_, AppState>,
    item: String,
) -> Result<Option<String>, String> {
    Ok(cache::get_foto(&state.app_data_dir, &item))
}

#[tauri::command]
fn cache_save_foto(
    state: tauri::State<'_, AppState>,
    item: String,
    src: String,
) -> Result<(), String> {
    cache::set_foto(&state.app_data_dir, &item, &src)
}

#[tauri::command]
fn cache_upsert_op(
    state: tauri::State<'_, AppState>,
    op: cache::PendingOp,
) -> Result<(), String> {
    cache::upsert_op(&state.app_data_dir, op)
}

#[tauri::command]
fn cache_set_op_flags(
    state: tauri::State<'_, AppState>,
    serie: String,
    mapics_ok: Option<bool>,
    sql_ok: Option<bool>,
) -> Result<(), String> {
    cache::set_op_flags(&state.app_data_dir, &serie, mapics_ok, sql_ok)
}

#[tauri::command]
fn cache_remove_op(
    state: tauri::State<'_, AppState>,
    serie: String,
) -> Result<(), String> {
    cache::remove_op(&state.app_data_dir, &serie)
}

#[tauri::command]
fn cache_mark_procesada(
    state: tauri::State<'_, AppState>,
    serie: String,
) -> Result<(), String> {
    cache::mark_procesada(&state.app_data_dir, &serie)
}

#[tauri::command]
fn cache_is_procesada(
    state: tauri::State<'_, AppState>,
    serie: String,
) -> Result<bool, String> {
    Ok(cache::is_procesada(&state.app_data_dir, &serie))
}

#[tauri::command]
async fn sync_buffer(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let dir = state.app_data_dir.clone();
    let (sent, series) = run_blocking(move || cache::sync_buffer(&dir, &cfg, &zone)).await?;
    Ok(json!({ "sent": sent, "series": series }))
}

#[tauri::command]
async fn sql_check_tables(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = run_blocking(move || sql::verificar_tablas(&cfg, &zone)).await?;
    Ok(json!({ "tables": rows }))
}

#[tauri::command]
async fn sql_create_tables(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rows = run_blocking(move || sql::crear_tablas_faltantes(&cfg, &zone)).await?;
    Ok(json!({ "tables": rows }))
}

#[tauri::command]
async fn sql_insert_resultado(
    state: tauri::State<'_, AppState>,
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
    let fecha = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    run_blocking(move || {
        sql::insert_resultado(
            &cfg,
            &zone,
            &fecha,
            &pedido,
            &serie,
            &resultado,
            &operador,
            &operador_admin,
            &comentario,
        )
    })
    .await
}

#[tauri::command]
async fn reimprimir(
    state: tauri::State<'_, AppState>,
    serie: String,
    operador_admin: String,
) -> Result<String, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();

    let fecha = chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let del = {
        let zone = zone.clone();
        let s2 = serie.clone();
        run_blocking(move || mapics::delete_kit(&zone, &s2)).await?
    };
    let ins = {
        let zone = zone.clone();
        let s2 = serie.clone();
        run_blocking(move || mapics::insert_kit(&zone, &s2)).await?
    };
    let resultado_serie = serie.clone();
    let resultado_fecha = fecha.clone();
    run_blocking({
        let cfg = cfg.clone();
        let zone = zone.clone();
        move || {
            sql::insert_resultado(
                &cfg,
                &zone,
                &resultado_fecha,
                "",
                &resultado_serie,
                "REIMPRESO",
                "N/A",
                &operador_admin,
                &format!("Serie reimpresa: {}", resultado_serie),
            )
        }
    })
    .await?;
    let error_serie = serie.clone();
    let error_fecha = fecha.clone();
    run_blocking({
        let cfg = cfg.clone();
        let zone = zone.clone();
        move || {
            sql::insert_error(
                &cfg,
                &zone,
                &error_fecha,
                "Reimpresión",
                &format!("Numero de serie reimpreso: {}", error_serie),
            )
        }
    })
    .await?;
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
            sql_check_serie_aprobada,
            sql_check_tables,
            sql_create_tables,
            mapics_precache,
            cache_snapshot,
            cache_get_kit,
            cache_save_kit,
            cache_get_foto,
            cache_save_foto,
            cache_upsert_op,
            cache_set_op_flags,
            cache_remove_op,
            cache_mark_procesada,
            cache_is_procesada,
            sync_buffer,
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