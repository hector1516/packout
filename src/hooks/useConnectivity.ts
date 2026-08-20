import { useCallback, useEffect, useRef, useState } from "react";
import { testZone } from "../lib/config";
import { cacheSnapshot, syncBuffer, type CacheSnapshot } from "../lib/packout";

export interface ConnectivityState {
  sqlOnline: boolean;
  mapicsOnline: boolean;
  checking: boolean;
  snapshot: CacheSnapshot | null;
  pendingCount: number;
}

export function useConnectivity() {
  const [state, setState] = useState<ConnectivityState>({
    sqlOnline: false,
    mapicsOnline: false,
    checking: false,
    snapshot: null,
    pendingCount: 0,
  });
  const syncingRef = useRef(false);

  const refresh = useCallback(async () => {
    setState((s) => ({ ...s, checking: true }));
    let sql = false;
    let mapics = false;
    try {
      const res = await testZone();
      sql = res.sql.ok;
      mapics = res.mapics.ok;
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

    if (sql && mapics && pending > 0 && !syncingRef.current) {
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
    });
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