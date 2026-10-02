mod cache;
mod config;
mod guard;
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
async fn sql_cleanup(
    state: tauri::State<'_, AppState>,
    dias: Option<i64>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let days = dias.unwrap_or(365);
    run_blocking(move || sql::limpiar_registros_antiguos(&cfg, &zone, days)).await
}

#[tauri::command]
async fn sql_reportes(
    state: tauri::State<'_, AppState>,
    tabla: Option<String>,
    desde: Option<String>,
    hasta: Option<String>,
    resultado: Option<String>,
    busqueda: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa configurada".to_string())?.clone();
    let t = tabla.unwrap_or_else(|| "resultados".into());
    let lim = limit.unwrap_or(200);
    let off = offset.unwrap_or(0);
    run_blocking(move || sql::consultar_reportes(&cfg, &zone, &t, desde, hasta, resultado, busqueda, lim, off)).await
}

#[tauri::command]
async fn sql_list_usuarios(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa".to_string())?.clone();
    let rows = run_blocking(move || sql::listar_usuarios(&cfg, &zone)).await?;
    Ok(json!({"rows": rows}))
}

#[tauri::command]
async fn sql_insert_usuario(state: tauri::State<'_, AppState>, no: String, nombre: String) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa".to_string())?.clone();
    run_blocking(move || sql::insertar_usuario(&cfg, &zone, &no, &nombre)).await?;
    Ok(json!({"ok": true}))
}

#[tauri::command]
async fn sql_update_usuario(state: tauri::State<'_, AppState>, no: String, nombre: String) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa".to_string())?.clone();
    run_blocking(move || sql::actualizar_usuario(&cfg, &zone, &no, &nombre)).await?;
    Ok(json!({"ok": true}))
}

#[tauri::command]
async fn sql_delete_usuario(state: tauri::State<'_, AppState>, no: String) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa".to_string())?.clone();
    run_blocking(move || sql::eliminar_usuario(&cfg, &zone, &no)).await?;
    Ok(json!({"ok": true}))
}

#[tauri::command]
async fn sql_list_admins(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa".to_string())?.clone();
    let rows = run_blocking(move || sql::listar_admins(&cfg, &zone)).await?;
    Ok(json!({"rows": rows}))
}

#[tauri::command]
async fn sql_insert_admin(state: tauri::State<'_, AppState>, no: String, nombre: String) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa".to_string())?.clone();
    run_blocking(move || sql::insertar_admin(&cfg, &zone, &no, &nombre)).await?;
    Ok(json!({"ok": true}))
}

#[tauri::command]
async fn sql_update_admin(state: tauri::State<'_, AppState>, no: String, nombre: String) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa".to_string())?.clone();
    run_blocking(move || sql::actualizar_admin(&cfg, &zone, &no, &nombre)).await?;
    Ok(json!({"ok": true}))
}

#[tauri::command]
async fn sql_delete_admin(state: tauri::State<'_, AppState>, no: String) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg.active().ok_or_else(|| "No hay zona activa".to_string())?.clone();
    run_blocking(move || sql::eliminar_admin(&cfg, &zone, &no)).await?;
    Ok(json!({"ok": true}))
}

#[tauri::command]
fn save_sound_file(
    state: tauri::State<AppState>,
    kind: String,
    source_path: String,
) -> Result<String, String> {
    let kind = if kind == "error" { "error" } else { "complete" };
    let mut cfg = load_config(&state)?;
    let sounds_dir = state.app_data_dir.join("sounds");
    std::fs::create_dir_all(&sounds_dir).map_err(|e| e.to_string())?;
    let target = sounds_dir.join(format!("{}.mp3", kind));
    std::fs::copy(&source_path, &target).map_err(|e| format!("No se pudo copiar el archivo: {}", e))?;
    let path = target.to_string_lossy().to_string();
    if kind == "error" {
        cfg.sound.error = path.clone();
    } else {
        cfg.sound.complete = path.clone();
    }
    config::save(&state.app_data_dir, &cfg)?;
    Ok(path)
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

/// Chequeo de disponibilidad SIN autenticacion.
///
/// Es lo que usa el polling cada 20s: solo abre el socket TCP del servidor,
/// nunca envia un LOGIN7. Asi una contrasena incorrecta no genera ningun
/// intento fallido contra SQL Server y la cuenta no se bloquea.
#[tauri::command]
async fn health_check(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let server = cfg.sql_for(Some(&zone)).server.clone();
    let mapics_server = zone.mapics.server.clone();

    let (sql_ping, mapics_ping) = tokio::join!(
        run_blocking({
            let server = server.clone();
            move || sql::ping(&server)
        }),
        run_blocking(move || mapics::ping(&mapics_server)),
    );
    let st = sql::guard().snapshot();

    Ok(json!({
        "sql": match sql_ping {
            Ok(s) => json!({"ok": true, "msg": s}),
            Err(e) => json!({"ok": false, "msg": e}),
        },
        "mapics": match mapics_ping {
            Ok(s) => json!({"ok": true, "msg": s}),
            Err(e) => json!({"ok": false, "msg": e}),
        },
        "guard": st,
    }))
}

/// Estado del circuito de proteccion de SQL (para mostrarlo en la UI).
#[tauri::command]
fn sql_guard_status() -> serde_json::Value {
    json!(sql::guard().snapshot())
}

/// Reinicia el circuito: lo pulsa el operador despues de corregir
/// usuario/contrasena. Hasta entonces NO se intenta ningun login mas.
#[tauri::command]
fn sql_guard_reset() -> serde_json::Value {
    sql::guard().reset();
    json!(sql::guard().snapshot())
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
fn restore_mapics_defaults(
    state: tauri::State<AppState>,
    zone_id: String,
) -> Result<AppConfig, String> {
    let mut cfg = load_config(&state)?;
    let zone = cfg
        .zones
        .iter_mut()
        .find(|z| z.id == zone_id)
        .ok_or_else(|| format!("Zona '{}' no encontrada", zone_id))?;
    config::restore_default_queries(zone);
    config::save(&state.app_data_dir, &cfg)?;
    Ok(cfg)
}

/// Importa todas las imagenes de una carpeta.
///
/// `on_event` reporta el avance en vivo. El nombre del archivo (sin
/// extension) es el codigo del item. Usa una sola conexion SQL para todo
/// el lote, asi que 500 imagenes cuestan 1 solo login.
#[tauri::command]
async fn import_images_from_folder(
    state: tauri::State<'_, AppState>,
    folder: String,
    recursive: Option<bool>,
    overwrite: Option<bool>,
    on_event: tauri::ipc::Channel<sql::ImportProgress>,
) -> Result<sql::ImportSummary, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let rec = recursive.unwrap_or(true);
    let ow = overwrite.unwrap_or(false);
    run_blocking(move || {
        sql::importar_imagenes_carpeta(&cfg, &zone, &folder, rec, ow, |p| {
            let _ = on_event.send(p.clone());
        })
    })
    .await
}

#[tauri::command]
async fn sql_scan_red(base_ip: String) -> Result<serde_json::Value, String> {
    let found = run_blocking(move || sql::scan_subnet(&base_ip)).await?;
    Ok(json!({ "servers": found, "count": found.len() }))
}

#[tauri::command]
async fn sql_list_databases(
    server: String,
    user: String,
    password: String,
) -> Result<serde_json::Value, String> {
    let dbs = run_blocking(move || sql::list_databases(&server, &user, &password)).await?;
    Ok(json!({ "databases": dbs, "count": dbs.len() }))
}

#[tauri::command]
async fn mapics_test(
    state: tauri::State<'_, AppState>,
    server: Option<String>,
    dsn: Option<String>,
    user: Option<String>,
    password: Option<String>,
) -> Result<serde_json::Value, String> {
    let cfg = load_config(&state)?;
    let zone = cfg
        .active()
        .ok_or_else(|| "No hay zona activa configurada".to_string())?
        .clone();
    let srv = server.unwrap_or_else(|| zone.mapics.server.clone());
    let d = dsn.unwrap_or_else(|| zone.mapics.dsn.clone());
    let u = user.unwrap_or_else(|| zone.mapics.user.clone());
    let p = password.unwrap_or_else(|| zone.mapics.password.clone());
    let res = run_blocking(move || mapics::test_connection_params(&d, &u, &p, &srv)).await;
    Ok(match res {
        Ok(r) => json!({
            "ok": true,
            "msg": format!("{} fila(s) devueltas", r.rows.len()),
            "query": mapics::TEST_QUERY,
            "columns": r.columns,
            "rows": r.rows,
        }),
        Err(e) => json!({"ok": false, "msg": e, "query": mapics::TEST_QUERY, "columns": [], "rows": []}),
    })
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
            save_sound_file,
            export_config,
            import_config,
            set_active_zone,
            test_zone,
            health_check,
            sql_guard_status,
            sql_guard_reset,
            mapics_test,
            restore_mapics_defaults,
            sql_scan_red,
            sql_list_databases,
            import_images_from_folder,
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
            sql_cleanup,
            sql_reportes,
            sql_list_usuarios,
            sql_insert_usuario,
            sql_update_usuario,
            sql_delete_usuario,
            sql_list_admins,
            sql_insert_admin,
            sql_update_admin,
            sql_delete_admin,
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