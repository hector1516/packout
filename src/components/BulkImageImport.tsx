import { useRef, useState } from "react";
import { open as dialogOpen } from "@tauri-apps/plugin-dialog";
import {
  importImagesFromFolder,
  type ImportProgress,
  type ImportSummary,
} from "../lib/packout";

type Fase = "idle" | "corriendo" | "listo" | "error";

function msAMmss(ms: number): string {
  const s = Math.max(0, Math.round(ms / 1000));
  const m = Math.floor(s / 60);
  const r = s % 60;
  return `${String(m).padStart(2, "0")}:${String(r).padStart(2, "0")}`;
}

/**
 * Importacion masiva de imagenes desde una carpeta.
 *
 * El nombre del archivo (sin extension) es el codigo del item, asi que una
 * carpeta con `CJAJUL8.png`, `CABLE-10.jpg`, etc. carga todo de una vez.
 */
export function BulkImageImport({
  onDone,
  compact,
}: {
  onDone?: () => void;
  compact?: boolean;
}) {
  const [fase, setFase] = useState<Fase>("idle");
  const [prog, setProg] = useState<ImportProgress | null>(null);
  const [resumen, setResumen] = useState<ImportSummary | null>(null);
  const [error, setError] = useState<string>("");
  const [recursivo, setRecursivo] = useState(true);
  const [sobrescribir, setSobrescribir] = useState(false);
  const [carpeta, setCarpeta] = useState<string>("");
  const rafRef = useRef<number | null>(null);
  const suaveRef = useRef<ImportProgress | null>(null);

  // Suaviza las actualizaciones para que el anillo no salte por cada imagen.
  function suave(p: ImportProgress) {
    suaveRef.current = p;
    const pintar = () => {
      if (suaveRef.current) setProg({ ...suaveRef.current });
      rafRef.current = null;
    };
    if (rafRef.current === null) {
      rafRef.current = window.setTimeout(pintar, 60);
    }
  }

  async function elegirCarpeta() {
    const dir = await dialogOpen({
      title: "Elige la carpeta con las imagenes de los items",
      directory: true,
      multiple: false,
    });
    if (typeof dir === "string") {
      setCarpeta(dir);
      setFase("idle");
      setResumen(null);
      setError("");
    }
  }

  async function iniciar() {
    if (!carpeta) {
      setError("Primero elige una carpeta");
      setFase("error");
      return;
    }
    setFase("corriendo");
    setError("");
    setResumen(null);
    setProg({
      processed: 0,
      total: 0,
      saved: 0,
      failed: 0,
      skipped: 0,
      current: "Buscando imagenes...",
      pct: 0,
      elapsedMs: 0,
      etaMs: null,
      speedPerSec: 0,
    });
    try {
      const r = await importImagesFromFolder(carpeta, suave, {
        recursive: recursivo,
        overwrite: sobrescribir,
      });
      setResumen(r);
      setFase(r.total === 0 ? "error" : "listo");
      if (r.total === 0) setError("No se encontraron imagenes en esa carpeta");
      onDone?.();
    } catch (e) {
      setError(String(e));
      setFase("error");
    }
  }

  const pct = Math.min(100, Math.max(0, prog?.pct ?? 0));
  const R = 62;
  const C = 2 * Math.PI * R;
  const activo = fase === "corriendo";

  if (fase === "idle" || (fase === "error" && !prog)) {
    return (
      <div className={`bulk-import ${compact ? "compact" : ""}`}>
        <h3>📁 Cargar imágenes desde carpeta</h3>
        <p className="muted">
          El <strong>nombre del archivo</strong> es el código del item. Ejemplo:
          una carpeta con <code>CJAJUL8.png</code> y <code>CABLE-10.jpg</code> los
          carga automáticamente.
        </p>
        <div className="bulk-picker">
          <button className="btn primary" onClick={elegirCarpeta}>
            📂 Elegir carpeta
          </button>
          {carpeta && <span className="bulk-path mono" title={carpeta}>{carpeta}</span>}
        </div>
        <div className="bulk-opts">
          <label className="col-toggle">
            <input
              type="checkbox"
              checked={recursivo}
              onChange={(e) => setRecursivo(e.currentTarget.checked)}
            />
            Incluir subcarpetas
          </label>
          <label className="col-toggle">
            <input
              type="checkbox"
              checked={sobrescribir}
              onChange={(e) => setSobrescribir(e.currentTarget.checked)}
            />
            Sobrescribir items que ya tienen foto
          </label>
        </div>
        {error && <p className="error-text">{error}</p>}
        <div className="modal-actions">
          <button
            className="btn primary big"
            onClick={iniciar}
            disabled={!carpeta}
          >
            ⚡ Cargar todas las imágenes
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className={`bulk-import running ${compact ? "compact" : ""}`}>
      <h3>
        {activo
          ? "⚡ Cargando imágenes..."
          : fase === "listo"
            ? "✅ ¡Carga completada!"
            : "❌ No se pudo completar"}
      </h3>

      <div className="bulk-stage">
        <div className="bulk-ring">
          <svg viewBox="0 0 150 150">
            <defs>
              <linearGradient id="bulkGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#38bdf8" />
                <stop offset="50%" stopColor="#6366f1" />
                <stop offset="100%" stopColor="#a855f7" />
              </linearGradient>
            </defs>
            <circle cx="75" cy="75" r={R} className="bulk-ring-track" />
            <circle
              cx="75"
              cy="75"
              r={R}
              className="bulk-ring-fill"
              strokeDasharray={C}
              strokeDashoffset={C * (1 - pct / 100)}
            />
          </svg>
          <div className="bulk-ring-label">
            <strong>{pct.toFixed(1)}%</strong>
            <span>
              {prog?.processed ?? 0} / {prog?.total ?? 0}
            </span>
          </div>
        </div>

        <div className="bulk-stats">
          <div className="bulk-stat">
            <span className="bulk-stat-k">Guardadas</span>
            <span className="bulk-stat-v ok">{prog?.saved ?? 0}</span>
          </div>
          <div className="bulk-stat">
            <span className="bulk-stat-k">Fallidas</span>
            <span className={`bulk-stat-v ${(prog?.failed ?? 0) > 0 ? "bad" : ""}`}>
              {prog?.failed ?? 0}
            </span>
          </div>
          <div className="bulk-stat">
            <span className="bulk-stat-k">Saltadas</span>
            <span className="bulk-stat-v">{prog?.skipped ?? 0}</span>
          </div>
          <div className="bulk-stat">
            <span className="bulk-stat-k">Transcurrido</span>
            <span className="bulk-stat-v">{msAMmss(prog?.elapsedMs ?? 0)}</span>
          </div>
          <div className="bulk-stat">
            <span className="bulk-stat-k">Restante</span>
            <span className="bulk-stat-v">
              {prog?.etaMs === null || prog?.etaMs === undefined
                ? "—"
                : msAMmss(prog.etaMs)}
            </span>
          </div>
          <div className="bulk-stat">
            <span className="bulk-stat-k">Velocidad</span>
            <span className="bulk-stat-v">
              {(prog?.speedPerSec ?? 0).toFixed(1)}/s
            </span>
          </div>
        </div>
      </div>

      <div className="bulk-bar">
        <div className="bulk-bar-fill" style={{ width: `${pct}%` }} />
      </div>

      <p className="bulk-current mono">
        {activo ? `📄 ${prog?.current ?? ""}` : ""}
      </p>

      {error && <p className="error-text">{error}</p>}

      {resumen && !activo && (
        <div className="bulk-result">
          <p>
            <strong>{resumen.total}</strong> imágenes procesadas en{" "}
            <strong>{msAMmss(resumen.elapsedMs)}</strong> —{" "}
            <span className="ok">{resumen.saved} guardadas</span>
            {resumen.failed > 0 && <span className="bad"> · {resumen.failed} con error</span>}
            {resumen.skipped > 0 && <span> · {resumen.skipped} ya existían</span>}
          </p>
          {resumen.errores.length > 0 && (
            <details className="bulk-errors">
              <summary>Ver errores ({resumen.errores.length})</summary>
              <ul>
                {resumen.errores.map((e, i) => (
                  <li key={i}>{e}</li>
                ))}
              </ul>
            </details>
          )}
        </div>
      )}

      <div className="modal-actions">
        <button
          className="btn"
          disabled={activo}
          onClick={() => {
            setFase("idle");
            setProg(null);
            setResumen(null);
            setError("");
          }}
        >
          {activo ? "Cargando..." : "Cargar otra carpeta"}
        </button>
      </div>
    </div>
  );
}
