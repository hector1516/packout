import { useEffect } from "react";
import { sqlCleanup } from "../lib/packout";

const KEY = "packout_last_cleanup";

export function useRetention() {
  useEffect(() => {
    const last = localStorage.getItem(KEY);
    const now = Date.now();
    const weekMs = 7 * 24 * 60 * 60 * 1000;
    if (last && now - Number(last) < weekMs) return;
    sqlCleanup(365)
      .then(() => localStorage.setItem(KEY, String(now)))
      .catch(() => {});
  }, []);
}
