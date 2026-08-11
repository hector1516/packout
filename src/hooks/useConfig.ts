import { useCallback, useEffect, useState } from "react";
import { getConfig, saveConfig, type AppConfig } from "../lib/config";

export function useConfig() {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const reload = useCallback(async () => {
    setLoading(true);
    try {
      setConfig(await getConfig());
      setError(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    reload();
  }, [reload]);

  const update = useCallback((next: AppConfig) => {
    setConfig(next);
  }, []);

  const save = useCallback(async () => {
    if (!config) return;
    try {
      await saveConfig(config);
      setError(null);
    } catch (e) {
      setError(String(e));
      throw e;
    }
  }, [config]);

  const active = config?.zones.find((z) => z.id === config.activeZone) ?? null;

  return { config, set: update, save, active, reload, loading, error };
}