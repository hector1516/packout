import { useCallback, useEffect, useRef, useState } from "react";
import { healthCheck } from "../lib/config";
import { cacheSnapshot, syncBuffer, type CacheSnapshot } from "../lib/packout";

export interface GuardStatus {
  blocked: boolean;
  authFailures: number;
  netFailures: number;
  maxAttempts: number;
  lastError: string;
  blockReason: string;
  blockCount: number;
}

export interface ConnectivityState {
  sqlOnline: boolean;
  mapicsOnline: boolean;
  checking: boolean;
  snapshot: CacheSnapshot | null;
  pendingCount: number;
  guard: GuardStatus | null;
}

/**
 * Solo hace ping (sin login) para saber si el servidor esta vivo.
 * Nunca autentica, por lo que un password incorrecto no puede bloquear
 * la cuenta de SQL Server.
 */
export function useConnectivity() {
  const [state, setState] = useState<ConnectivityState>({
    sqlOnline: false,
    mapicsOnline: false,
    checking: false,
    snapshot: null,
    pendingCount: 0,
    guard: null,
  });
  const syncingRef = useRef(false);
  const inFlightRef = useRef(false);

  const refresh = useCallback(async () => {
    // Sin pestanas en paralelo: cada tick dispara como maximo un chequeo.
    if (inFlightRef.current) return;
    inFlightRef.current = true;
    setState((s) => ({ ...s, checking: true }));
    let sql = false;
    let mapics = false;
    let guard: GuardStatus | null = null;
    try {
      const res = await healthCheck();
      sql = res.sql.ok;
      mapics = res.mapics.ok;
      guard = res.guard;
    } catch {
      sql = false;
      mapics = false;
    }
    let snapshot: CacheSnapshot | null = null;
    try {
      snapshot = await cacheSnapshot();
    } catch {
      snapshot = null;
    }
    const pending = snapshot ? snapshot.cola.length : 0;

    // Con el circuito bloqueado no se intenta sincronizar: cada sync es un
    // login mas y no queremos sumar intentos fallidos.
    if (sql && mapics && pending > 0 && !syncingRef.current && !guard?.blocked) {
      syncingRef.current = true;
      try {
        await syncBuffer();
        snapshot = await cacheSnapshot();
      } catch {
        /* reintenta en el próximo ciclo */
      } finally {
        syncingRef.current = false;
      }
    }

    setState({
      sqlOnline: sql,
      mapicsOnline: mapics,
      checking: false,
      snapshot,
      pendingCount: snapshot ? snapshot.cola.length : 0,
      guard,
    });
    inFlightRef.current = false;
  }, []);

  useEffect(() => {
    refresh();
    const t = setInterval(refresh, 20000);
    return () => clearInterval(t);
  }, [refresh]);

  const syncNow = useCallback(async () => {
    await syncBuffer();
    await refresh();
  }, [refresh]);

  return { ...state, refresh, syncNow };
}
