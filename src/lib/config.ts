import { invoke } from "@tauri-apps/api/core";

export interface SqlDb {
  server: string;
  database: string;
  user: string;
  password: string;
  driver: string;
}

export interface TableNames {
  resultados: string;
  errores: string;
  usuarios: string;
  admin: string;
  recientes: string;
  itemImages: string;
}

export interface MapicsZone {
  server: string;
  dsn: string;
  user: string;
  password: string;
  queryKit: string;
  queryInsert: string;
  queryDelete: string;
  queryBuffer: string;
}

export interface Zone {
  id: string;
  nombre: string;
  estacion: string;
  tables: TableNames;
  mapics: MapicsZone;
  sql?: SqlDb;
}

export interface AppConfig {
  activeZone: string;
  sql: SqlDb;
  zones: Zone[];
  bufferKits?: number;
}

export interface TestResult {
  zone: string;
  sql: { ok: boolean; msg: string };
  mapics: { ok: boolean; msg: string };
}

export async function getConfig(): Promise<AppConfig> {
  return invoke<AppConfig>("get_config");
}

export async function saveConfig(config: AppConfig): Promise<void> {
  return invoke("save_config", { config });
}

export async function exportConfig(path: string): Promise<void> {
  return invoke("export_config", { path });
}

export async function importConfig(path: string): Promise<AppConfig> {
  return invoke<AppConfig>("import_config", { path });
}

export async function setActiveZone(zoneId: string): Promise<AppConfig> {
  return invoke<AppConfig>("set_active_zone", { zoneId });
}

export async function testZone(): Promise<TestResult> {
  return invoke<TestResult>("test_zone");
}