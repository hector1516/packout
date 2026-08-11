import { useEffect, useState } from "react";
import { SettingsPanel } from "./components/SettingsPanel";
import { MainScreen } from "./components/MainScreen";
import { useUpdater } from "./hooks/useUpdater";
import "./App.css";

function App() {
  const [page, setPage] = useState<"main" | "settings">("main");
  const [splash, setSplash] = useState(true);
  const updater = useUpdater();

  useEffect(() => {
    const t = setTimeout(() => setSplash(false), 1400);
    return () => clearTimeout(t);
  }, []);

  return (
    <div className="app">
      {splash && (
        <div className="splash">
          <div className="splash-inner">
            <img src="/eccsa.png" alt="ECCSA" className="splash-img" draggable={false} />
            <span className="splash-title">PACKOUT</span>
            <span className="muted">Cargando...</span>
          </div>
        </div>
      )}
      {page === "main" ? (
        <MainScreen onOpenSettings={() => setPage("settings")} />
      ) : (
        <SettingsPanel onBack={() => setPage("main")} />
      )}
      {!splash && updater.state.phase === "available" && (
        <div className="update-banner">
          <span>
            Nueva versión <strong>{updater.state.update.version}</strong> disponible
          </span>
          <div className="update-actions">
            <button className="btn subtle" onClick={() => updater.check({ manual: true })}>
              Revisar
            </button>
            <button className="btn" onClick={updater.install}>
              Actualizar
            </button>
          </div>
        </div>
      )}
      {!splash && updater.state.phase === "downloading" && (
        <div className="update-banner">
          <span>Descargando actualización…</span>
          <span className="muted">{(updater.state as { percent: number }).percent}%</span>
        </div>
      )}
      {!splash && updater.state.phase === "installing" && (
        <div className="update-banner">
          <span>Instalando actualización…</span>
        </div>
      )}
      {!splash && updater.state.phase === "error" && (
        <div className="update-banner error-banner">
          <span>Actualización falló: {updater.state.message}</span>
          <button className="btn subtle" onClick={() => updater.check({ manual: true })}>
            Reintentar
          </button>
        </div>
      )}
      {!splash && updater.state.phase === "idle" && (
        <button
          className="update-check"
          onClick={() => updater.check({ manual: true })}
        >
          Buscar actualizaciones
        </button>
      )}
    </div>
  );
}

export default App;