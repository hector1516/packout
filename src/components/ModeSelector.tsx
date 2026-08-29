import { saveConfig, type AppConfig } from "../lib/config";

export function ModeSelector({
  config,
  onChosen,
}: {
  config: AppConfig;
  onChosen: (next: AppConfig) => void;
}) {
  const choose = async (modo: "operacion" | "administracion") => {
    const next = { ...config, modo, modoElegido: true };
    await saveConfig(next);
    onChosen(next);
  };

  return (
    <div className="mode-selector">
      <div className="mode-selector-inner">
        <img src="/eccsa.png" alt="ECCSA" className="mode-logo" draggable={false} />
        <h1 className="mode-title">Bienvenido a Packout</h1>
        <p className="muted">Elige cómo usar esta instalación</p>
        <div className="mode-cards">
          <button className="mode-card" onClick={() => choose("operacion")}>
            <span className="mode-icon">⚙️</span>
            <h2>Modo Operación</h2>
            <p className="muted">Pantalla de escaneo para línea de producción. Contador, fotos, buffer y celebración.</p>
            <span className="btn primary">Usar Operación</span>
          </button>
          <button className="mode-card admin" onClick={() => choose("administracion")}>
            <span className="mode-icon">📊</span>
            <h2>Modo Administración</h2>
            <p className="muted">Reportes, fotos por item, usuarios y administradores. Todo visual y exportable.</p>
            <span className="btn primary">Usar Administración</span>
          </button>
        </div>
        <p className="muted" style={{ fontSize: 12 }}>Podrás cambiarlo en Configuración en cualquier momento.</p>
      </div>
    </div>
  );
}
