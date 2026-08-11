import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";

export interface UpdateInfo {
  version: string;
  current_version: string;
}

type ProgressEvent =
  | { event: "Started"; data: { contentLength: number | null } }
  | { event: "Progress"; data: { chunkLength: number } }
  | { event: "Finished" };

export type UpdaterState =
  | { phase: "checking" }
  | { phase: "idle" }
  | { phase: "available"; update: UpdateInfo }
  | { phase: "downloading"; percent: number }
  | { phase: "installing" }
  | { phase: "done" }
  | { phase: "error"; message: string };

export function useUpdater() {
  const [state, setState] = useState<UpdaterState>({ phase: "checking" });
  const checkedRef = useRef(false);

  const check = useCallback(async (opts?: { manual?: boolean }) => {
    const manual = opts?.manual ?? false;
    if (!manual) setState({ phase: "checking" });
    try {
      const info = await invoke<UpdateInfo | null>("check_update");
      if (info) {
        setState({ phase: "available", update: info });
      } else {
        setState({ phase: "idle" });
      }
    } catch (e) {
      if (manual) setState({ phase: "error", message: String(e) });
      else setState({ phase: "idle" });
    }
  }, []);

  const install = useCallback(async () => {
    setState({ phase: "downloading", percent: 0 });
    const channel = new Channel<ProgressEvent>();
    channel.onmessage = (ev) => {
      switch (ev.event) {
        case "Started":
          setState({ phase: "downloading", percent: 0 });
          break;
        case "Progress":
          setState((s) =>
            s.phase === "downloading"
              ? { phase: "downloading", percent: (s.percent + ev.data.chunkLength) % 100 }
              : s,
          );
          break;
        case "Finished":
          setState({ phase: "installing" });
          break;
      }
    };
    try {
      await invoke("install_update", { onEvent: channel });
      setState({ phase: "done" });
    } catch (e) {
      setState({ phase: "error", message: String(e) });
    }
  }, []);

  useEffect(() => {
    if (checkedRef.current) return;
    checkedRef.current = true;
    check();
  }, [check]);

  return { state, check, install };
}