use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlDb {
    pub server: String,
    pub database: String,
    pub user: String,
    pub password: String,
    #[serde(default = "default_sql_driver")]
    pub driver: String,
}

fn default_sql_driver() -> String {
    "SQL Server".into()
}

impl Default for SqlDb {
    fn default() -> Self {
        SqlDb {
            server: "10.96.16.114".into(),
            database: "hussmann_insight".into(),
            user: "HInsightUser".into(),
            password: "CAMBIAME".into(),
            driver: default_sql_driver(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableNames {
    pub resultados: String,
    pub errores: String,
    pub usuarios: String,
    pub admin: String,
    pub recientes: String,
    #[serde(default = "default_item_images_table")]
    pub item_images: String,
}

fn default_item_images_table() -> String {
    "PackoutItemsImgIMX".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapicsZone {
    pub server: String,
    pub dsn: String,
    pub user: String,
    pub password: String,
    pub query_kit: String,
    pub query_insert: String,
    pub query_delete: String,
    #[serde(default = "default_query_buffer")]
    pub query_buffer: String,
}

fn default_query_buffer() -> String {
    "SELECT DISTINCT IMANUSE FROM XACHGMEP.EPCIMAGE WHERE IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR = 7) AND IMANUSE > '{SERIE}' AND IMAESKI <> 'Disabled' ORDER BY IMANUSE FETCH FIRST {LIMIT} ROWS ONLY".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    pub id: String,
    pub nombre: String,
    pub estacion: String,
    pub tables: TableNames,
    pub mapics: MapicsZone,
    #[serde(default)]
    pub sql: Option<SqlDb>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundConfig {
    #[serde(default = "default_sound_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub complete: String,
    #[serde(default)]
    pub error: String,
}

fn default_sound_enabled() -> bool {
    true
}

impl Default for SoundConfig {
    fn default() -> Self {
        SoundConfig {
            enabled: default_sound_enabled(),
            complete: String::new(),
            error: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub active_zone: String,
    pub sql: SqlDb,
    pub zones: Vec<Zone>,
    #[serde(default = "default_buffer_kits")]
    pub buffer_kits: u32,
    #[serde(default = "SoundConfig::default")]
    pub sound: SoundConfig,
}

fn default_buffer_kits() -> u32 {
    30
}

impl AppConfig {
    pub fn active(&self) -> Option<&Zone> {
        self.zones.iter().find(|z| z.id == self.active_zone)
    }

    pub fn sql_for<'a>(&'a self, zone: Option<&'a Zone>) -> &'a SqlDb {
        zone.and_then(|z| z.sql.as_ref()).unwrap_or(&self.sql)
    }
}

pub const DEFAULT_CONFIG: &str = r#"{
  "activeZone": "imx",
  "bufferKits": 30,
  "sound": {
    "enabled": true,
    "complete": "",
    "error": ""
  },
  "sql": {
    "server": "10.96.16.114",
    "database": "hussmann_insight",
    "user": "HInsightUser",
    "password": "CAMBIAME",
    "driver": "SQL Server"
  },
  "zones": [
    {
      "id": "imx",
      "nombre": "IMX · Linea 7",
      "estacion": "ESTPACK01",
      "tables": {
        "resultados": "PackoutResultadosIMX",
        "errores": "PackoutErrIMX",
        "usuarios": "PackoutUsrIMX",
        "admin": "PackoutAdminIMX",
        "recientes": "PackoutResViewIMX",
        "itemImages": "PackoutItemsImgIMX"
      },
      "mapics": {
        "server": "prod.hussmann.com",
        "dsn": "datatest",
        "user": "DATRINS",
        "password": "CAMBIAME",
        "queryKit": "SELECT A.IMANUES, A.IMANULI, A.IMAORDE, A.IMANUSE, A.IMAKIT, A.IMACONF, A.IMADATE, ifnull(B.EPCTIPO,'') as EPCTIPO, ifnull(B.EPCFECHA,0) as EPCFECHA, ifnull(B.EPCHORA,0) as EPCHORA, ifnull(B.EPCNIMPR,'') as EPCNIMPR, A.IMADATE as FechaRegEPC, A.IMAESKI FROM XACHGMEP.EPCIMAGE A left outer JOIN XACHGMEP.EPCBITA B ON A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT AND A.IMANUES = B.EPCESTAC AND A.IMANULI = B.EPCLINEA WHERE A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR = 7) AND A.IMAESKI <> 'Disabled' AND A.IMANUSE = '{SERIE}' ORDER BY A.IMADATE DESC",
        "queryInsert": "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE, C.INSMOO, C.INSSER, C.INSSCU, 'PACKOUT COMPLETE', '{ESTACION}', CURRENT DATE, CURRENT TIME, 'ECCSA' FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '{SERIE}' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')",
        "queryDelete": "DELETE FROM XACHGMEP.FESRLKIT WHERE KPSRLN = '{SERIE}'",
        "queryBuffer": "SELECT DISTINCT IMANUSE FROM XACHGMEP.EPCIMAGE WHERE IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR = 7) AND IMANUSE > '{SERIE}' AND IMAESKI <> 'Disabled' ORDER BY IMANUSE FETCH FIRST {LIMIT} ROWS ONLY"
      }
    }
  ]
}"#;

pub fn config_path(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join("packout.config.json")
}

pub fn load(app_data_dir: &PathBuf) -> Result<AppConfig, String> {
    let path = config_path(app_data_dir);
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&path, DEFAULT_CONFIG).map_err(|e| e.to_string())?;
        serde_json::from_str(DEFAULT_CONFIG).map_err(|e| e.to_string())
    } else {
        let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).map_err(|e| e.to_string())
    }
}

pub fn save(app_data_dir: &PathBuf, config: &AppConfig) -> Result<(), String> {
    let path = config_path(app_data_dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let pretty = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, pretty).map_err(|e| e.to_string())
}

pub fn export_to(path: &std::path::Path, config: &AppConfig) -> Result<(), String> {
    let pretty = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, pretty).map_err(|e| e.to_string())
}

pub fn import_from(path: &std::path::Path) -> Result<AppConfig, String> {
    if !path.exists() {
        return Err(format!("No existe el archivo: {:?}", path));
    }
    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let cfg: AppConfig = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    if cfg.zones.is_empty() {
        return Err("El archivo de configuración no tiene zonas definidas".into());
    }
    Ok(cfg)
}

pub fn render(query: &str, serie: &str, estacion: &str) -> String {
    query
        .replace("{SERIE}", serie)
        .replace("{ESTACION}", estacion)
}