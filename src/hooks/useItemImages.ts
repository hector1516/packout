import { useCallback, useEffect, useState } from "react";
import { sqlItemImage, type KitItem } from "../lib/packout";

export interface ItemImage {
  key: string;
  src: string | null;
}

export function useItemImages(items: KitItem[]) {
  const [images, setImages] = useState<ItemImage[]>([]);
  const [loading, setLoading] = useState(false);
  const [nonce, setNonce] = useState(0);

  const reload = useCallback(() => setNonce((n) => n + 1), []);

  const keys = items.map((it) => it.key).join("|");

  useEffect(() => {
    let cancelled = false;
    setLoading(true);

    const load = async () => {
      const out: ItemImage[] = [];
      for (const it of items) {
        if (cancelled) return;
        let src: string | null = null;
        try {
          src = await sqlItemImage(it.desc);
        } catch {
          src = null;
        }
        out.push({ key: it.key, src });
      }
      if (!cancelled) {
        setImages(out);
        setLoading(false);
      }
    };

    load();
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [keys, nonce]);

  return { images, loading, reload };
}