import { invoke } from "@tauri-apps/api/core";

export type KitRow = Record<string, string>;

export interface QueryKitResult {
  rows: KitRow[];
  count: number;
}

export interface HistorialResult {
  rows: KitRow[];
  count: number;
}

export interface LookupResult {
  rows: KitRow[];
  found: boolean;
}

export async function mapicsQueryKit(serie: string): Promise<QueryKitResult> {
  return invoke<QueryKitResult>("mapics_query_kit", { serie });
}

export async function mapicsInsertKit(serie: string): Promise<{ affected: number }> {
  return invoke("mapics_insert_kit", { serie });
}

export async function mapicsDeleteKit(serie: string): Promise<{ affected: number }> {
  return invoke("mapics_delete_kit", { serie });
}

export async function sqlHistorial(top = 10): Promise<HistorialResult> {
  return invoke<HistorialResult>("sql_historial", { top });
}

export async function sqlRecientes(top = 20): Promise<HistorialResult> {
  return invoke<HistorialResult>("sql_recientes", { top });
}

export async function reimprimir(serie: string, operadorAdmin: string): Promise<void> {
  return invoke("reimprimir", { serie, operadorAdmin });
}

export async function sqlItemImage(item: string): Promise<string | null> {
  return invoke<string | null>("sql_item_image", { item });
}

export async function sqlSaveItemImage(item: string, imagen: string): Promise<void> {
  return invoke("sql_save_item_image", { item, imagen });
}

export async function sqlDeleteItemImage(item: string): Promise<void> {
  return invoke("sql_delete_item_image", { item });
}

export interface ListItemImagesResult {
  rows: KitRow[];
  count: number;
}

export async function sqlListItemImages(): Promise<ListItemImagesResult> {
  return invoke<ListItemImagesResult>("sql_list_item_images");
}

export async function sqlInsertResultado(params: {
  pedido: string;
  serie: string;
  resultado: string;
  operador: string;
  operadorAdmin: string;
  comentario: string;
}): Promise<void> {
  return invoke("sql_insert_resultado", params);
}

export async function sqlInsertError(titulo: string, desc: string): Promise<void> {
  return invoke("sql_insert_error", { titulo, desc });
}

export async function sqlLogin(no: string): Promise<LookupResult> {
  return invoke<LookupResult>("sql_login", { no });
}

export async function sqlCheckOperator(no: string): Promise<LookupResult> {
  return invoke<LookupResult>("sql_check_operator", { no });
}

export async function sqlCheckSerieAprobada(serie: string): Promise<LookupResult> {
  return invoke<LookupResult>("sql_check_serie_aprobada", { serie });
}

export interface CachedItem {
  key: string;
  desc: string;
  scanned: boolean;
}

export interface CachedKit {
  serie: string;
  pedido: string;
  items: CachedItem[];
  image?: string | null;
}

export interface PendingOp {
  tipo: string;
  serie: string;
  pedido: string;
  resultado: string;
  operador: string;
  operadorAdmin: string;
  comentario: string;
  fecha: string;
  mapicsOk: boolean;
  sqlOk: boolean;
}

export interface CacheSnapshot {
  kits: { serie: string; pedido: string; items: number; image: boolean }[];
  cola: PendingOp[];
  procesadas: string[];
}

export async function mapicsPrecache(serie: string, limit?: number): Promise<{ added: string[] }> {
  return invoke("mapics_precache", { serie, limit });
}

export async function cacheSnapshot(): Promise<CacheSnapshot> {
  return invoke<CacheSnapshot>("cache_snapshot");
}

export async function cacheGetKit(serie: string): Promise<CachedKit & { found: boolean }> {
  return invoke("cache_get_kit", { serie });
}

export async function cacheSaveKit(
  serie: string,
  pedido: string,
  items: CachedItem[],
  image?: string | null,
): Promise<void> {
  return invoke("cache_save_kit", { serie, pedido, items, image });
}

export async function cacheGetFoto(item: string): Promise<string | null> {
  return invoke<string | null>("cache_get_foto", { item });
}

export async function cacheSaveFoto(item: string, src: string): Promise<void> {
  return invoke("cache_save_foto", { item, src });
}

export async function cacheUpsertOp(op: PendingOp): Promise<void> {
  return invoke("cache_upsert_op", { op });
}

export async function cacheSetOpFlags(
  serie: string,
  mapicsOk?: boolean | null,
  sqlOk?: boolean | null,
): Promise<void> {
  return invoke("cache_set_op_flags", { serie, mapicsOk, sqlOk });
}

export async function cacheRemoveOp(serie: string): Promise<void> {
  return invoke("cache_remove_op", { serie });
}

export async function cacheMarkProcesada(serie: string): Promise<void> {
  return invoke("cache_mark_procesada", { serie });
}

export async function cacheIsProcesada(serie: string): Promise<boolean> {
  return invoke<boolean>("cache_is_procesada", { serie });
}

export async function syncBuffer(): Promise<{ sent: number; series: string[] }> {
  return invoke("sync_buffer");
}

export interface TableCheck {
  table: string;
  exists: boolean;
  isView: boolean;
  columns: string[];
  missing: string[];
  ok: boolean;
}

export async function sqlCheckTables(): Promise<{ tables: TableCheck[] }> {
  return invoke("sql_check_tables");
}

export async function sqlCreateTables(): Promise<{ tables: TableCheck[] }> {
  return invoke("sql_create_tables");
}

export const KIT_SERIE_PREFIXES = ["MY", "my"];
export const OPEN_PAREN = "(";

export function normalizeScan(raw: string): string {
  return raw.replace(/[()]/g, "").trim().toUpperCase();
}

export function looksLikeSerial(code: string): boolean {
  return KIT_SERIE_PREFIXES.some((p) => code.startsWith(p));
}

export interface KitItem {
  key: string;
  desc: string;
  label: string;
  scanned: boolean;
}

export function buildItems(rows: KitRow[]): { items: KitItem[]; count: number } {
  const items: KitItem[] = [];
  let count = 0;
  for (const r of rows) {
    const aeski = r["IMAESKI"] ?? "";
    const conf = r["IMACONF"] ?? "";
    const nimp = r["EPCNIMPR"] ?? "";
    const fecha = r["EPCFECHA"] ?? "";
    const manuse = r["IMANUSE"] ?? "";
    const makit = r["IMAKIT"] ?? "";
    const estacion = r["IMANUES"] ?? "";

    if (aeski === "Disabled") continue;

    if (conf === "Y") {
      if (nimp.includes("A")) {
        count += 1;
        if (fecha.length > 1) {
          if (["90", "790", "810", "910"].includes(estacion)) {
            const key = `${manuse.replace(/ /g, "")}${nimp.replace(/ /g, "")}`;
            items.push({ key, desc: makit.replace(/ /g, ""), label: key, scanned: false });
          } else {
            const key = `${manuse.replace(/ /g, "")}XX1`;
            items.push({ key, desc: makit.replace(/ /g, ""), label: key, scanned: false });
          }
        }
      }
    } else {
      const key = `${manuse.replace(/ /g, "")}XX3`;
      items.push({ key, desc: makit.replace(/ /g, ""), label: key, scanned: false });
    }
  }
  return { items, count };
}