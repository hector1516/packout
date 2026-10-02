import { useCallback, useEffect, useRef, useState } from "react";
import {
  buildItems,
  cacheGetKit,
  cacheIsProcesada,
  cacheMarkProcesada,
  cacheRemoveOp,
  cacheSaveKit,
  cacheUpsertOp,
  looksLikeSerial,
  mapicsInsertKit,
  mapicsPrecache,
  normalizeScan,
  reimprimir,
  sqlCheckOperator,
  sqlCheckSerieAprobada,
  sqlHistorial,
  sqlInsertError,
  sqlInsertResultado,
  sqlLogin,
  sqlRecientes,
  type CachedItem,
  type KitItem,
  type KitRow,
  type PendingOp,
} from "../lib/packout";
import { mapicsQueryKit } from "../lib/packout";
import { useConfig } from "./useConfig";

export type FlowStatus = "idle" | "kit" | "approved" | "done";

/** Que esta esperando la app en este momento. */
export type PasoEsperado = "serie" | "item" | "gafete" | "listo";

export const TEXTO_PASO: Record<PasoEsperado, string> = {
  serie: "NÚMERO DE SERIE",
  item: "ITEM DEL KIT",
  gafete: "GAFETE",
  listo: "NINGUNO",
};

/** AvisoShown cuando el operador escanea algo que no corresponde. */
export interface FlowNotice {
  titulo: string;
  motivo: string;
  esperado: PasoEsperado;
  recibido: string;
  /** El escaneo no se aplico. */
  rejected: boolean;
}

export interface FlowState {
  status: FlowStatus;
  serie: string;
  pedido: string;
  items: KitItem[];
  remaining: number;
  resultado: "PENDIENTE" | "APROBADO" | "";
  operatorNo: string;
  operatorAdmin: string;
  message: string;
  error: string;
  historial: KitRow[];
  aviso: FlowNotice | null;
}

/** El paso que la app espera segun el estado actual del flujo. */
export function pasoEsperado(state: Pick<FlowState, "status" | "items">): PasoEsperado {
  if (state.status === "idle") return "serie";
  if (state.status === "kit") {
    return state.items.length > 0 ? "item" : "serie";
  }
  if (state.status === "approved") return "gafete";
  return "listo";
}

function blank(): FlowState {
  return {
    status: "idle",
    serie: "",
    pedido: "",
    items: [],
    remaining: 0,
    resultado: "",
    operatorNo: "",
    operatorAdmin: "",
    message: "",
    error: "",
    historial: [],
    aviso: null,
  };
}

export function usePackoutFlow() {
  const [state, setState] = useState<FlowState>(blank);
  const { config } = useConfig();
  const mapicsOkRef = useRef(false);

  const refreshHistorial = useCallback(async () => {
    try {
      const h = await sqlHistorial(10);
      setState((s) => ({ ...s, historial: h.rows }));
    } catch (e) {
      setState((s) => ({ ...s, error: String(e) }));
    }
  }, []);

  useEffect(() => {
    refreshHistorial();
  }, [refreshHistorial]);

  const setMessage = useCallback((message: string) => {
    setState((s) => ({ ...s, message }));
    setTimeout(() => setState((s) => (s.message === message ? { ...s, message: "" } : s)), 4000);
  }, []);

  const setAviso = useCallback((aviso: FlowNotice | null) => {
    setState((s) => ({ ...s, aviso }));
  }, []);

  const clearAviso = useCallback(() => {
    setState((s) => ({ ...s, aviso: null }));
  }, []);

  // El aviso se apaga solo para que el operador pueda seguir escaneando.
  useEffect(() => {
    if (!state.aviso) return;
    const t = setTimeout(() => {
      setState((s) => (s.aviso === state.aviso ? { ...s, aviso: null } : s));
    }, 6000);
    return () => clearTimeout(t);
  }, [state.aviso]);

  const logError = useCallback(async (msg: string) => {
    setState((s) => ({ ...s, error: msg }));
    try {
      await sqlInsertError("Packout", msg);
    } catch {
      /* ignora si no se puede registrar */
    }
  }, []);

  const loadKit = useCallback(
    async (serie: string) => {
      setState((s) => ({ ...s, serie, resultado: "PENDIENTE", message: "Consultando MAPICS..." }));
      let already = false;
      try {
        const check = await sqlCheckSerieAprobada(serie);
        already = check.found;
      } catch {
        try {
          already = await cacheIsProcesada(serie);
        } catch {
          already = false;
        }
      }
      if (already) {
        setState((s) => ({ ...s, serie: "N/A", message: `La serie ${serie} ya fue aprobada` }));
        return;
      }

      try {
        const res = await mapicsQueryKit(serie);
        if (res.count === 0) {
          setState((s) => ({ ...s, serie: "N/A", message: "Sin items en MAPICS para " + serie }));
          return;
        }
        const { items, count } = buildItems(res.rows);
        const pedido = res.rows[0]?.["IMAORDE"] ?? "";
        setState((s) => ({
          ...s,
          status: "kit",
          pedido,
          items,
          remaining: count,
          resultado: "PENDIENTE",
          message: `Kit cargado: ${items.length} items`,
          error: "",
        }));
        const cached: CachedItem[] = items.map((it) => ({
          key: it.key,
          desc: it.desc,
          scanned: it.scanned,
        }));
        cacheSaveKit(serie, pedido, cached, null).catch(() => {});
        const bufferKits = config?.bufferKits ?? 30;
        mapicsPrecache(serie, bufferKits).catch(() => {});
      } catch (onlineErr) {
        try {
          const cached = await cacheGetKit(serie);
          if (!cached.found || cached.items.length === 0) {
            setState((s) => ({
              ...s,
              serie: "N/A",
              message: `No disponible offline: ${serie}`,
            }));
            return;
          }
          const items: KitItem[] = cached.items.map((c) => ({
            key: c.key,
            desc: c.desc,
            label: c.key,
            scanned: c.scanned,
          }));
          setState((s) => ({
            ...s,
            status: "kit",
            pedido: cached.pedido,
            items,
            remaining: items.filter((i) => !i.scanned).length,
            resultado: "PENDIENTE",
            message: `Kit cargado offline: ${items.length} items`,
            error: "",
          }));
        } catch {
          logError(`Error en mapics_query_kit: ${onlineErr}`);
          setState((s) => ({ ...s, message: "Error al consultar MAPICS" }));
        }
      }
    },
    [config?.bufferKits, logError],
  );

  const approve = useCallback(async () => {
    const now = new Date().toLocaleString("es-MX");
    mapicsOkRef.current = false;
    try {
      await mapicsInsertKit(state.serie);
      mapicsOkRef.current = true;
    } catch (e) {
      const op: PendingOp = {
        tipo: "aprobado",
        serie: state.serie,
        pedido: state.pedido,
        resultado: "APROBADO",
        operador: "",
        operadorAdmin: state.operatorAdmin,
        comentario: "",
        fecha: now,
        mapicsOk: false,
        sqlOk: false,
      };
      cacheUpsertOp(op).catch(() => {});
    }
    cacheMarkProcesada(state.serie).catch(() => {});
    setState((s) => ({
      ...s,
      status: "approved",
      resultado: "APROBADO",
      message: "APROBADO — escanea tu gafete para registrar",
      error: "",
    }));
  }, [state.serie, state.pedido, state.operatorAdmin]);

  const scanItem = useCallback(
    (key: string) => {
      const idx = state.items.findIndex((it) => it.key === key);
      if (idx === -1) {
        setMessage(`"${key}" no está en la lista del kit`);
        setAviso({
          titulo: "Ese item no es del kit",
          motivo: `"${key}" no aparece en la lista del equipo ${state.serie || ""}`.trim(),
          esperado: "item",
          recibido: key,
          rejected: true,
        });
        return;
      }
      if (state.items[idx].scanned) {
        setMessage(`"${key}" ya fue escaneado`);
        setAviso({
          titulo: "Item repetido",
          motivo: `"${key}" ya se había escaneado. Faltan ${state.remaining} item(s).`,
          esperado: "item",
          recibido: key,
          rejected: true,
        });
        return;
      }
      const items = state.items.map((it, i) =>
        i === idx ? { ...it, scanned: true } : it,
      );
      const remaining = Math.max(0, state.remaining - 1);
      if (remaining === 0) {
        setState((s) => ({ ...s, items, remaining, aviso: null }));
        approve();
      } else {
        setState((s) => ({ ...s, items, remaining, message: `Faltan ${remaining}`, aviso: null }));
      }
    },
    [state.items, state.remaining, state.serie, setMessage, setAviso, approve],
  );

  const feed = useCallback(
    async (raw: string) => {
      const code = normalizeScan(raw);
      if (code.length < 2) return;

      switch (state.status) {
        case "idle": {
          if (!looksLikeSerial(code)) {
            setMessage(`Serial inválido: ${code}`);
            setAviso({
              titulo: "Eso no es un número de serie",
              motivo: `Escaneaste "${code}". Primero se escanea el número de serie del equipo.`,
              esperado: "serie",
              recibido: code,
              rejected: true,
            });
            return;
          }
          setAviso(null);
          await loadKit(code);
          break;
        }
        case "kit": {
          // Si parece otra serie, avisar en vez de tratarlo como item.
          if (looksLikeSerial(code) && code !== state.serie) {
            setMessage("Esa es otra serie, no un item");
            setAviso({
              titulo: "Esperaba un item",
              motivo: `Escaneaste la serie "${code}", pero el kit abierto es ${state.serie}. Todavía faltan ${state.remaining} item(s).`,
              esperado: "item",
              recibido: code,
              rejected: true,
            });
            break;
          }
          scanItem(code);
          break;
        }
        case "approved": {
          // Si escanea una serie o un item cuando ya toca el gafete, avisar.
          if (looksLikeSerial(code)) {
            setMessage("Esperaba el gafete, no una serie");
            setAviso({
              titulo: "Solo falta tu gafete",
              motivo: `Escaneaste "${code}". El equipo ${state.serie} ya está completo: escanea tu gafete de colaborador.`,
              esperado: "gafete",
              recibido: code,
              rejected: true,
            });
            break;
          }
          const now = new Date().toLocaleString("es-MX");
          let op:
            | { found: boolean; operador: string }
            | undefined;
          try {
            const res = await sqlCheckOperator(code);
            op = { found: res.found, operador: code };
          } catch {
            op = { found: true, operador: code };
          }
          if (op && !op.found) {
            setMessage(`Colaborador no registrado: ${code}`);
            setAviso({
              titulo: "Colaborador no registrado",
              motivo: `El gafete "${code}" no existe en la lista de colaboradores. Pide a Administración que lo registre.`,
              esperado: "gafete",
              recibido: code,
              rejected: true,
            });
            break;
          }
          const operador = code;
          try {
            await sqlInsertResultado({
              pedido: state.pedido,
              serie: state.serie,
              resultado: "APROBADO",
              operador,
              operadorAdmin: state.operatorAdmin,
              comentario: "",
            });
            cacheRemoveOp(state.serie).catch(() => {});
          } catch {
            const mapicsOk = mapicsOkRef.current;
            const pending: PendingOp = {
              tipo: "aprobado",
              serie: state.serie,
              pedido: state.pedido,
              resultado: "APROBADO",
              operador,
              operadorAdmin: state.operatorAdmin,
              comentario: "",
              fecha: now,
              mapicsOk,
              sqlOk: false,
            };
            cacheUpsertOp(pending).catch(() => {});
          }
          setState((s) => ({ ...s, operatorNo: operador, status: "done", message: "Registrado. Listo para el siguiente." }));
          refreshHistorial();
          setTimeout(() => setState(blank()), 2500);
          break;
        }
        default:
          break;
      }
    },
    [state, loadKit, scanItem, logError, refreshHistorial],
  );

  const recordPending = useCallback(
    async (admin: string, comentario: string) => {
      if (!state.serie) return;
      const faltantes = state.items.filter((i) => !i.scanned).map((i) => i.key);
      const now = new Date().toLocaleString("es-MX");
      try {
        await sqlInsertResultado({
          pedido: state.pedido,
          serie: state.serie,
          resultado: "PENDIENTE",
          operador: "",
          operadorAdmin: admin,
          comentario: `${comentario} items: ${faltantes.join(", ")}`,
        });
      } catch {
        const pending: PendingOp = {
          tipo: "pendiente",
          serie: state.serie,
          pedido: state.pedido,
          resultado: "PENDIENTE",
          operador: "",
          operadorAdmin: admin,
          comentario: `${comentario} items: ${faltantes.join(", ")}`,
          fecha: now,
          mapicsOk: true,
          sqlOk: false,
        };
        cacheUpsertOp(pending).catch(() => {});
      }
      setState((s) => ({ ...s, status: "done", message: "Pendiente registrado" }));
      refreshHistorial();
      setTimeout(() => setState(blank()), 2500);
    },
    [state, refreshHistorial, logError],
  );

  const recordManual = useCallback(
    async (code: string, operadorAdmin: string) => {
      const serie = normalizeScan(code);
      if (!looksLikeSerial(serie)) {
        setMessage(`Serial inválido: ${serie}`);
        return;
      }
      const now = new Date().toLocaleString("es-MX");
      try {
        await sqlInsertResultado({
          pedido: "",
          serie,
          resultado: "MANUAL",
          operador: "N/A",
          operadorAdmin,
          comentario: `Manual: ${serie}`,
        });
      } catch {
        const pending: PendingOp = {
          tipo: "manual",
          serie,
          pedido: "",
          resultado: "MANUAL",
          operador: "N/A",
          operadorAdmin,
          comentario: `Manual: ${serie}`,
          fecha: now,
          mapicsOk: true,
          sqlOk: false,
        };
        cacheUpsertOp(pending).catch(() => {});
      }
      setState((s) => ({ ...s, serie, status: "idle" as FlowStatus }));
      await loadKit(serie);
    },
    [loadKit, setMessage, logError],
  );

  const reprint = useCallback(
    async (serie: string, operadorAdmin: string) => {
      const s = normalizeScan(serie);
      if (!s) return "";
      try {
        await reimprimir(s, operadorAdmin);
        setMessage(`Serie reimpresa: ${s}`);
      } catch (e) {
        logError(`Error en reimpresión de ${s}: ${e}`);
      }
      return s;
    },
    [setMessage, logError],
  );

  const listRecientes = useCallback(async (top = 20) => {
    try {
      const h = await sqlRecientes(top);
      return h.rows;
    } catch (e) {
      logError(`Error consultando series recientes: ${e}`);
      return [];
    }
  }, [logError]);

  const loginAdmin = useCallback(
    async (no: string): Promise<boolean> => {
      try {
        const res = await sqlLogin(no);
        if (res.found) {
          setState((s) => ({ ...s, operatorAdmin: no }));
          return true;
        }
        return false;
      } catch (e) {
        logError(`Error consultando admin: ${e}`);
        return false;
      }
    },
    [logError],
  );

  useEffect(() => {
    if (!state.operatorAdmin) return;
    const t = setTimeout(() => {
      setState((s) => ({ ...s, operatorAdmin: "" }));
    }, 10 * 60 * 1000);
    return () => clearTimeout(t);
  }, [state.operatorAdmin]);

  const reset = useCallback(() => {
    setState((s) => ({ ...s, ...blank(), operatorAdmin: s.operatorAdmin }));
  }, []);

  const clearError = useCallback(() => {
    setState((s) => ({ ...s, error: "" }));
  }, []);

  return {
    state,
    feed,
    recordPending,
    recordManual,
    reprint,
    listRecientes,
    loginAdmin,
    reset,
    clearError,
    clearAviso,
    refreshHistorial,
  };
}