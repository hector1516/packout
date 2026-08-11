import { useEffect, useRef, useState } from "react";
import { usePackoutFlow, type FlowStatus } from "../hooks/usePackoutFlow";
import { useConfig } from "../hooks/useConfig";
import { useItemImages } from "../hooks/useItemImages";
import { OperatorModal, PendingModal, ReprintModal, ImagesModal } from "./modals";

type ModalKind = "none" | "login" | "manual" | "reprint" | "pending" | "images";

const GENERIC_IMAGE =
  "data:image/svg+xml;utf8," +
  encodeURIComponent(
    `<svg xmlns="http://www.w3.org/2000/svg" width="400" height="400"><rect width="400" height="400" fill="#1c2432"/><rect x="1" y="1" width="398" height="398" fill="none" stroke="#2a3448" stroke-width="2"/><rect x="80" y="70" width="240" height="180" rx="10" fill="#141b28"/><circle cx="200" cy="130" r="30" fill="#2b5bd7"/><rect x="120" y="205" width="160" height="14" rx="7" fill="#3a4a6b"/><rect x="130" y="235" width="140" height="10" rx="5" fill="#2a3448"/><text x="200" y="320" text-anchor="middle" font-family="Segoe UI, sans-serif" font-size="22" fill="#9fb0c7">ITEM</text></svg>`,
  );

export function MainScreen({
  onOpenSettings,
}: {
  onOpenSettings: () => void;
}) {
  const { state, feed, recordPending, recordManual, reprint, listRecientes, loginAdmin, reset, clearError } =
    usePackoutFlow();
  const { config } = useConfig();
  const { images, loading: imagesLoading, reload: reloadImages } = useItemImages(state.items);
  const [modal, setModal] = useState<ModalKind>("none");
  const [scan, setScan] = useState("");
  const scanRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const focus = () => scanRef.current?.focus();
    focus();
    window.addEventListener("click", focus);
    return () => window.removeEventListener("click", focus);
  }, []);

  const activeZone = config
    ? config.zones.find((z) => z.id === config.activeZone) ?? null
    : null;

  const submitScan = () => {
    if (!scan) return;
    feed(scan);
    setScan("");
  };

  const statusLabel: Record<FlowStatus, string> = {
    idle: "Escanea un serial",
    kit: "Escanea los items del kit",
    approved: "Escanea tu gafete de operador",
    done: "Registrado",
  };

  const idle = state.status === "idle" || state.items.length === 0;

  return (
    <div className="screen">
      <header className="topbar">
        <div className="brand">
          <img src="/hussmann.png" alt="Hussmann" className="brand-logo" draggable={false} />
          <span className="brand-title">PACKOUT</span>
          <span className="muted">
            Zona: {activeZone?.nombre ?? "—"} · Estación: {activeZone?.estacion ?? "—"}
          </span>
        </div>
        <div className="topbar-actions">
          {state.operatorAdmin && (
            <span className="chip">Admin: {state.operatorAdmin}</span>
          )}
          <button className="btn" onClick={() => setModal("login")}>
            Ingreso
          </button>
          <button className="btn" onClick={() => setModal("pending")}>
            Pendiente
          </button>
          <button className="btn" onClick={() => setModal("manual")}>
            Manual
          </button>
          <button className="btn" onClick={() => setModal("reprint")}>
            Reimprimir
          </button>
          <button className="btn" onClick={() => setModal("images")}>
            Imágenes
          </button>
          <button className="btn" onClick={onOpenSettings}>
            Configuración
          </button>
        </div>
      </header>

      <input
        ref={scanRef}
        className="scan-input"
        value={scan}
        onChange={(e) => setScan(e.target.value)}
        onKeyDown={(e) => e.key === "Enter" && submitScan()}
        placeholder="Scanner o teclado… (Enter)"
        autoFocus
      />

      <div className={`status-banner status-${state.status}`}>
        <strong>{statusLabel[state.status]}</strong>
        {state.message && <span className="muted"> · {state.message}</span>}
      </div>

      {state.error && (
        <div className="error-banner">
          <span className="error-text">{state.error}</span>
          <button className="btn subtle" onClick={clearError}>
            Cerrar
          </button>
        </div>
      )}

      <div className="main-grid">
        <section className="panel photo-panel">
          <div className="panel-header">
            <h3>Kit actual</h3>
            <span className="muted">
              Serie: {state.serie} · Pedido: {state.pedido}
            </span>
            {!idle && <span className="chip big-chip">{state.remaining} restantes</span>}
          </div>

          {idle ? (
            <div className="empty-state">
              <img src="/engrane.png" alt="" className="empty-logo-img" draggable={false} />
              <span className="muted">Escanea o captura una serie para cargar el kit</span>
            </div>
          ) : (
            <div className="photo-grid">
              {imagesLoading && state.items.length > 0 && (
                <span className="muted">Cargando fotos…</span>
              )}
              {images.map((img) => {
                const item = state.items.find((it) => it.key === img.key);
                const scanned = item?.scanned ?? false;
                return (
                  <div
                    key={img.key}
                    className={`photo-card${scanned ? " scanned" : ""}`}
                  >
                    <div className="photo-card-img">
                      {img.src ? (
                        <img src={img.src} alt={img.key} />
                      ) : (
                        <img src={GENERIC_IMAGE} alt={img.key} className="generic-img" />
                      )}
                    </div>
                    <div className="photo-card-caption">
                      <span className="item-code">{img.key}</span>
                      <span className="item-desc">
                        {scanned ? "✓ escaneado" : (item?.desc ?? "")}
                      </span>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </section>

        <aside className="panel hist-panel">
          <div className="panel-header">
            <h3>Historial</h3>
            <button className="btn subtle" onClick={reset}>
              Limpiar
            </button>
          </div>
          <div className="hist-scroll">
        <table className="table">
          <thead>
            <tr>
              <th>Serie</th>
              <th>Resultado</th>
              <th>Operador</th>
            </tr>
          </thead>
          <tbody>
            {state.historial.map((r, i) => (
              <tr key={i}>
                <td>{r["SERIE"] ?? r["serie"] ?? "—"}</td>
                <td>{r["RESULTADO"] ?? r["resultado"] ?? "—"}</td>
                <td>{r["OPERADOR"] ?? r["operador"] ?? "—"}</td>
              </tr>
            ))}
            {state.historial.length === 0 && (
              <tr>
                <td colSpan={3} className="muted">
                  sin registros
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
        </aside>
      </div>

      {modal === "login" && (
        <OperatorModal
          title="Ingreso de operador/admin"
          inputLabel="Gafete"
          onOk={async (no) => {
            const ok = await loginAdmin(no);
            if (ok) setModal("none");
          }}
          onClose={() => setModal("none")}
        />
      )}
      {modal === "manual" && (
        <OperatorModal
          title="Registro manual"
          inputLabel="Serial a registrar"
          onOk={(s) => {
            recordManual(s, state.operatorAdmin);
            setModal("none");
          }}
          onClose={() => setModal("none")}
        />
      )}
      {modal === "reprint" && (
        <ReprintModal
          listRecientes={listRecientes}
          onSave={(s) => {
            reprint(s, state.operatorAdmin);
            setModal("none");
          }}
          onClose={() => setModal("none")}
        />
      )}
      {modal === "pending" && (
        <PendingModal
          onSave={(admin, comentario) => {
            recordPending(admin, comentario);
            setModal("none");
          }}
          onClose={() => setModal("none")}
        />
      )}
      {modal === "images" && (
        <ImagesModal
          onChanged={reloadImages}
          onClose={() => setModal("none")}
        />
      )}
    </div>
  );
}