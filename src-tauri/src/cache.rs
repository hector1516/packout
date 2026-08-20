use crate::config::{AppConfig, Zone};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedKit {
    pub serie: String,
    pub pedido: String,
    pub items: Vec<CachedItem>,
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedItem {
    pub key: String,
    pub desc: String,
    pub scanned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingOp {
    pub tipo: String,
    pub serie: String,
    pub pedido: String,
    pub resultado: String,
    pub operador: String,
    pub operador_admin: String,
    pub comentario: String,
    pub fecha: String,
    #[serde(default)]
    pub mapics_ok: bool,
    #[serde(default)]
    pub sql_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    #[serde(default)]
    pub kits: Vec<CachedKit>,
    #[serde(default)]
    pub cola: Vec<PendingOp>,
    #[serde(default)]
    pub procesadas: Vec<String>,
    #[serde(default)]
    pub fotos: BTreeMap<String, String>,
    #[serde(default)]
    pub buffer_kits: u32,
    #[serde(default)]
    pub max_fotos: u32,
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            kits: Vec::new(),
            cola: Vec::new(),
            procesadas: Vec::new(),
            fotos: BTreeMap::new(),
            buffer_kits: 30,
            max_fotos: 20,
        }
    }
}

pub fn cache_path(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join("packout.cache.json")
}

pub fn load(app_data_dir: &PathBuf) -> Cache {
    let path = cache_path(app_data_dir);
    match fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
        Err(_) => Cache::default(),
    }
}

pub fn save(app_data_dir: &PathBuf, cache: &Cache) -> Result<(), String> {
    let path = cache_path(app_data_dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let pretty = serde_json::to_string_pretty(cache).map_err(|e| e.to_string())?;
    fs::write(&path, pretty).map_err(|e| e.to_string())
}

pub fn upsert_op(app_data_dir: &PathBuf, op: PendingOp) -> Result<(), String> {
    let mut cache = load(app_data_dir);
    if let Some(existing) = cache.cola.iter_mut().find(|e| e.serie == op.serie) {
        *existing = op;
    } else {
        cache.cola.push(op);
    }
    save(app_data_dir, &cache)
}

pub fn set_op_flags(
    app_data_dir: &PathBuf,
    serie: &str,
    mapics_ok: Option<bool>,
    sql_ok: Option<bool>,
) -> Result<(), String> {
    let mut cache = load(app_data_dir);
    if let Some(op) = cache.cola.iter_mut().find(|e| e.serie == serie) {
        if let Some(v) = mapics_ok {
            op.mapics_ok = v;
        }
        if let Some(v) = sql_ok {
            op.sql_ok = v;
        }
    }
    save(app_data_dir, &cache)
}

pub fn remove_op(app_data_dir: &PathBuf, serie: &str) -> Result<(), String> {
    let mut cache = load(app_data_dir);
    cache.cola.retain(|e| e.serie != serie);
    save(app_data_dir, &cache)
}

pub fn mark_procesada(app_data_dir: &PathBuf, serie: &str) -> Result<(), String> {
    let mut cache = load(app_data_dir);
    if !cache.procesadas.iter().any(|s| s == serie) {
        cache.procesadas.push(serie.to_string());
    }
    save(app_data_dir, &cache)
}

pub fn is_procesada(app_data_dir: &PathBuf, serie: &str) -> bool {
    load(app_data_dir).procesadas.iter().any(|s| s == serie)
}

pub fn get_kit(app_data_dir: &PathBuf, serie: &str) -> Option<CachedKit> {
    load(app_data_dir)
        .kits
        .into_iter()
        .find(|k| k.serie == serie)
}

pub fn set_kit(app_data_dir: &PathBuf, kit: CachedKit) -> Result<(), String> {
    let mut cache = load(app_data_dir);
    cache.kits.retain(|k| k.serie != kit.serie);
    cache.kits.push(kit);
    if cache.kits.len() > cache.buffer_kits.max(1) as usize {
        cache.kits.remove(0);
    }
    save(app_data_dir, &cache)
}

pub fn get_foto(app_data_dir: &PathBuf, item: &str) -> Option<String> {
    load(app_data_dir).fotos.get(item).cloned()
}

pub fn set_foto(app_data_dir: &PathBuf, item: &str, src: &str) -> Result<(), String> {
    let mut cache = load(app_data_dir);
    cache.fotos.insert(item.to_string(), src.to_string());
    while cache.fotos.len() > cache.max_fotos.max(1) as usize {
        if let Some(eldest) = cache.fotos.keys().next().cloned() {
            cache.fotos.remove(&eldest);
        } else {
            break;
        }
    }
    save(app_data_dir, &cache)
}

pub fn precache(
    app_data_dir: &PathBuf,
    zone: &Zone,
    from: &str,
    limit: u32,
) -> Result<Vec<String>, String> {
    let serials = crate::mapics::query_buffer_serials(zone, from, limit)?;
    let mut added = Vec::new();
    let mut cache = load(app_data_dir);
    let max = limit.max(1);
    for serie in serials {
        if cache.kits.len() as u32 >= max && !cache.kits.iter().any(|k| k.serie == serie) {
            break;
        }
        if cache.kits.iter().any(|k| k.serie == serie) {
            continue;
        }
        if cache.procesadas.iter().any(|s| s.as_str() == serie) {
            continue;
        }
        match crate::mapics::query_kit(zone, &serie) {
            Ok(rows) if !rows.is_empty() => {
                let pedido = rows[0].get("IMAORDE").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let items = build_cached_items(&rows);
                cache.kits.push(CachedKit {
                    serie: serie.clone(),
                    pedido,
                    items,
                    image: None,
                });
                added.push(serie.clone());
            }
            _ => {}
        }
        if cache.kits.len() as u32 >= max {
            break;
        }
    }
    while cache.kits.len() as u32 > max {
        cache.kits.remove(0);
    }
    save(app_data_dir, &cache)?;
    Ok(added)
}

fn build_cached_items(rows: &[Value]) -> Vec<CachedItem> {
    let mut items = Vec::new();
    for r in rows {
        let aeski = r["IMAESKI"].as_str().unwrap_or("");
        let conf = r["IMACONF"].as_str().unwrap_or("");
        let nimp = r["EPCNIMPR"].as_str().unwrap_or("");
        let fecha = r["EPCFECHA"].as_str().unwrap_or("");
        let manuse = r["IMANUSE"].as_str().unwrap_or("");
        let makit = r["IMAKIT"].as_str().unwrap_or("");
        let estacion = r["IMANUES"].as_str().unwrap_or("");
        if aeski == "Disabled" {
            continue;
        }
        if conf == "Y" {
            if nimp.contains('A') && fecha.len() > 1 {
                let key = if ["90", "790", "810", "910"].contains(&estacion) {
                    format!("{}{}", manuse.replace(' ', ""), nimp.replace(' ', ""))
                } else {
                    format!("{}XX1", manuse.replace(' ', ""))
                };
                items.push(CachedItem {
                    key,
                    desc: makit.replace(' ', ""),
                    scanned: false,
                });
            }
        } else {
            items.push(CachedItem {
                key: format!("{}XX3", manuse.replace(' ', "")),
                desc: makit.replace(' ', ""),
                scanned: false,
            });
        }
    }
    items
}

pub fn sync_buffer(
    app_data_dir: &PathBuf,
    cfg: &AppConfig,
    zone: &Zone,
) -> Result<(usize, Vec<String>), String> {
    let sql_online = crate::sql::test_connection(cfg, Some(zone)).is_ok();
    let mapics_online = crate::mapics::test_connection(zone).is_ok();
    if !sql_online || !mapics_online {
        return Ok((0, Vec::new()));
    }
    let mut cache = load(app_data_dir);
    let mut sent = Vec::new();
    let mut i = 0;
    while i < cache.cola.len() {
        let mut op = cache.cola[i].clone();
        if op.mapics_ok {
            match crate::sql::insert_resultado(
                cfg,
                zone,
                &op.fecha,
                &op.pedido,
                &op.serie,
                &op.resultado,
                &op.operador,
                &op.operador_admin,
                &op.comentario,
            ) {
                Ok(()) => op.sql_ok = true,
                Err(_) => {}
            }
        } else {
            match crate::mapics::insert_kit(zone, &op.serie) {
                Ok(_) => op.mapics_ok = true,
                Err(_) => {}
            }
        }
        if op.mapics_ok && op.sql_ok {
            cache.cola.remove(i);
            sent.push(op.serie.clone());
        } else {
            cache.cola[i] = op;
            i += 1;
        }
    }
    save(app_data_dir, &cache)?;
    Ok((sent.len(), sent))
}