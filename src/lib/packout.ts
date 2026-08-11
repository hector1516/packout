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