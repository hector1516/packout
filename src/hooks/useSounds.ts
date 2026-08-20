import { useCallback } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { useConfig } from "./useConfig";

export function useSounds() {
  const { config } = useConfig();

  const play = useCallback(
    (kind: "complete" | "error") => {
      const sound = config?.sound;
      if (!sound?.enabled) return;
      const path = kind === "complete" ? sound.complete : sound.error;
      if (!path) return;
      try {
        const audio = new Audio(convertFileSrc(path));
        audio.play().catch(() => {});
      } catch {
        /* ignora si no se puede reproducir */
      }
    },
    [config?.sound],
  );

  return play;
}