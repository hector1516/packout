import {
  pasoEsperado,
  TEXTO_PASO,
  type FlowState,
  type PasoEsperado,
} from "../hooks/usePackoutFlow";

const ORDEN: PasoEsperado[] = ["serie", "item", "gafete", "listo"];

const ICONO: Record<PasoEsperado, string> = {
  serie: "🔢",
  item: "📦",
  gafete: "🎫",
  listo: "✅",
};

/**
 * Indicador de pasos gigante: en que paso va y que se espera ahora.
 * Se lee a distancia, que es como lo usa el operador en la linea.
 */
export function FlowSteps({ state }: { state: FlowState }) {
  const actual = pasoEsperado(state);
  const idxActual = ORDEN.indexOf(actual);
  const escaneados = state.items.filter((i) => i.scanned).length;
  const total = state.items.length;

  return (
    <div className="flow-steps">
      {ORDEN.map((p, i) => {
        const activo = p === actual;
        const hecho = i < idxActual || (p === "listo" && state.status === "done");
        return (
          <div
            key={p}
            className={`flow-step ${activo ? "active" : ""} ${hecho ? "done" : ""}`}
          >
            <div className="flow-step-dot">
              {hecho ? "✓" : <span>{ICONO[p]}</span>}
            </div>
            <div className="flow-step-body">
              <span className="flow-step-k">Paso {i + 1}</span>
              <span className="flow-step-t">{TEXTO_PASO[p]}</span>
              {p === "item" && total > 0 && (
                <span className="flow-step-sub">
                  {escaneados} de {total}
                </span>
              )}
            </div>
          </div>
        );
      })}

      <div className={`flow-wait wait-${actual}`}>
        <span className="flow-wait-pulse" />
        <div className="flow-wait-text">
          <span className="flow-wait-k">ESCANEA AHORA</span>
          <strong className="flow-wait-v">{TEXTO_PASO[actual]}</strong>
          {actual === "item" && state.remaining > 0 && (
            <span className="flow-wait-sub">
              Faltan {state.remaining} item{state.remaining === 1 ? "" : "s"}
            </span>
          )}
          {actual === "serie" && (
            <span className="flow-wait-sub">El código del equipo (ej. MY12345)</span>
          )}
          {actual === "gafete" && (
            <span className="flow-wait-sub">Tu gafete de colaborador</span>
          )}
        </div>
      </div>
    </div>
  );
}

/**
 * Aviso a pantalla completa cuando el operador escanea algo que no
 * corresponde. Se auto-cierra (6s).
 */
export function ScanNotice({
  state,
  onClose,
}: {
  state: FlowState;
  onClose: () => void;
}) {
  const a = state.aviso;
  if (!a) return null;
  return (
    <div className="scan-warning" onClick={onClose}>
      <div className="scan-warning-inner">
        <div className="scan-warning-icon">⚠️</div>
        <h2 className="scan-warning-title">{a.titulo}</h2>
        <div className="scan-warning-esperaba">
          <span className="scan-warning-k">Se esperaba</span>
          <strong>{TEXTO_PASO[a.esperado]}</strong>
        </div>
        <div className="scan-warning-recibido">
          <span className="scan-warning-k">Escaneaste</span>
          <strong className="mono">{a.recibido}</strong>
        </div>
        <p className="scan-warning-motivo">{a.motivo}</p>
        <div className="scan-warning-flecha">
          <span>👇</span>
          <div>
            <span className="scan-warning-k">Sigue con</span>
            <strong>{TEXTO_PASO[a.esperado]}</strong>
          </div>
        </div>
        <button className="btn scan-warning-btn">Entendido</button>
      </div>
    </div>
  );
}
