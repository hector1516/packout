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

export interface SoundConfig {
  enabled: boolean;
  complete: string;
  error: string;
}

export interface AppConfig {
  activeZone: string;
  sql: SqlDb;
  zones: Zone[];
  bufferKits?: number;
  sound?: SoundConfig;
  modo?: string;
  modoElegido?: boolean;
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

export interface GuardStatus {
  blocked: boolean;
  authFailures: number;
  netFailures: number;
  maxAttempts: number;
  lastError: string;
  blockReason: string;
  blockCount: number;
}

export interface HealthCheckResult {
  sql: { ok: boolean; msg: string };
  mapics: { ok: boolean; msg: string };
  guard: GuardStatus;
}

/** Ping sin autenticacion: seguro para polling. */
export async function healthCheck(): Promise<HealthCheckResult> {
  return invoke<HealthCheckResult>("health_check");
}

export async function sqlGuardStatus(): Promise<GuardStatus> {
  return invoke<GuardStatus>("sql_guard_status");
}

/** Reinicia el circuito de proteccion tras corregir el password. */
export async function sqlGuardReset(): Promise<GuardStatus> {
  return invoke<GuardStatus>("sql_guard_reset");
}

export interface ScanRedResult {
  servers: string[];
  count: number;
}

export async function sqlScanRed(baseIp: string): Promise<ScanRedResult> {
  return invoke<ScanRedResult>("sql_scan_red", { baseIp });
}

export interface ListDatabasesResult {
  databases: string[];
  count: number;
}

export async function sqlListDatabases(
  server: string,
  user: string,
  password: string,
): Promise<ListDatabasesResult> {
  return invoke<ListDatabasesResult>("sql_list_databases", { server, user, password });
}

export interface MapicsTestResult {
  ok: boolean;
  msg: string;
  query: string;
  columns: string[];
  rows: Array<Record<string, string>>;
}

export async function mapicsTest(params?: {
  server?: string;
  dsn?: string;
  user?: string;
  password?: string;
}): Promise<MapicsTestResult> {
  return invoke<MapicsTestResult>("mapics_test", params ?? {});
}

export async function restoreMapicsDefaults(zoneId: string): Promise<AppConfig> {
  return invoke<AppConfig>("restore_mapics_defaults", { zoneId });
}

export async function saveSoundFile(kind: "complete" | "error", sourcePath: string): Promise<string> {
  return invoke<string>("save_sound_file", { kind, sourcePath });
}