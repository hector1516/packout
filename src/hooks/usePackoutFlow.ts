import { useCallback, useEffect, useState } from "react";
import {
  buildItems,
  looksLikeSerial,
  mapicsInsertKit,
  normalizeScan,
  reimprimir,
  sqlCheckOperator,
  sqlHistorial,
  sqlInsertError,
  sqlInsertResultado,
  sqlLogin,
  sqlRecientes,
  type KitItem,
  type KitRow,
} from "../lib/packout";
import { mapicsQueryKit } from "../lib/packout";

export type FlowStatus = "idle" | "kit" | "approved" | "done";

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
  };
}

export function usePackoutFlow() {
  const [state, setState] = useState<FlowState>(blank);

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
      } catch (e) {
        logError(`Error en mapics_query_kit: ${e}`);
        setState((s) => ({ ...s, message: "Error al consultar MAPICS" }));
      }
    },
    [logError],
  );

  const approve = useCallback(async () => {
    try {
      await mapicsInsertKit(state.serie);
      setState((s) => ({
        ...s,
        status: "approved",
        resultado: "APROBADO",
        message: "APROBADO — escanea tu gafete para registrar",
        error: "",
      }));
    } catch (e) {
      logError(`Error en mapics_insert_kit: ${e}`);
      setState((s) => ({ ...s, message: "Error al insertar en MAPICS" }));
    }
  }, [state.serie, logError]);

  const scanItem = useCallback(
    (key: string) => {
      const idx = state.items.findIndex((it) => it.key === key);
      if (idx === -1) {
        setMessage(`"${key}" no está en la lista del kit`);
        return;
      }
      if (state.items[idx].scanned) {
        setMessage(`"${key}" ya fue escaneado`);
        return;
      }
      const items = state.items.map((it, i) =>
        i === idx ? { ...it, scanned: true } : it,
      );
      const remaining = Math.max(0, state.remaining - 1);
      if (remaining === 0) {
        setState((s) => ({ ...s, items, remaining }));
        approve();
      } else {
        setState((s) => ({ ...s, items, remaining, message: `Faltan ${remaining}` }));
      }
    },
    [state.items, state.remaining, setMessage, approve],
  );

  const feed = useCallback(
    async (raw: string) => {
      const code = normalizeScan(raw);
      if (code.length < 2) return;

      switch (state.status) {
        case "idle": {
          if (!looksLikeSerial(code)) {
            setMessage(`Serial inválido: ${code}`);
            return;
          }
          await loadKit(code);
          break;
        }
        case "kit": {
          scanItem(code);
          break;
        }
        case "approved": {
          try {
            const res = await sqlCheckOperator(code);
            if (res.found) {
              await sqlInsertResultado({
                pedido: state.pedido,
                serie: state.serie,
                resultado: "APROBADO",
                operador: code,
                operadorAdmin: state.operatorAdmin,
                comentario: "",
              });
              setState((s) => ({ ...s, operatorNo: code, status: "done", message: "Registrado. Listo para el siguiente." }));
              refreshHistorial();
              setTimeout(() => setState(blank()), 2500);
            } else {
              setMessage(`Operador no registrado: ${code}`);
            }
          } catch (e) {
            logError(`Error al registrar aprobado: ${e}`);
          }
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
      try {
        await sqlInsertResultado({
          pedido: state.pedido,
          serie: state.serie,
          resultado: "PENDIENTE",
          operador: "",
          operadorAdmin: admin,
          comentario: `${comentario} items: ${faltantes.join(", ")}`,
        });
        setState((s) => ({ ...s, status: "done", message: "Pendiente registrado" }));
        refreshHistorial();
        setTimeout(() => setState(blank()), 2500);
      } catch (e) {
        logError(`Error al registrar pendiente: ${e}`);
      }
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
      try {
        await sqlInsertResultado({
          pedido: "",
          serie,
          resultado: "MANUAL",
          operador: "N/A",
          operadorAdmin,
          comentario: `Manual: ${serie}`,
        });
      } catch (e) {
        logError(`Error registrando manual: ${e}`);
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
    refreshHistorial,
  };
}