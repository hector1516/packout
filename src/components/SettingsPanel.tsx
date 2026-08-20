import { useState } from "react";
import { save as dialogSave, open as dialogOpen } from "@tauri-apps/plugin-dialog";
import {
  setActiveZone,
  testZone,
  exportConfig,
  importConfig,
  saveConfig,
  type AppConfig,
  type TestResult,
  type Zone,
} from "../lib/config";
import { useConfig } from "../hooks/useConfig";
import type { useUpdater } from "../hooks/useUpdater";
import { SoundModal, TablesModal } from "./modals";

function Field({
  label,
  value,
  onChange,
  mono,
  rows,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
  mono?: boolean;
  rows?: number;
}) {
  return (
    <label className="field">
      <span>{label}</span>
      {rows ? (
        <textarea
          className={mono ? "mono" : undefined}
          rows={rows}
          value={value}
          onChange={(e) => onChange(e.currentTarget.value)}
        />
      ) : (
        <input
          className={mono ? "mono" : undefined}
          value={value}
          onChange={(e) => onChange(e.currentTarget.value)}
        />
      )}
    </label>
  );
}

export function SettingsPanel({
  onBack,
  updater,
}: {
  onBack?: () => void;
  updater?: ReturnType<typeof useUpdater>;
}) {
  const { config, set: setConfig, save, reload, loading, error } = useConfig();
  const [status, setStatus] = useState<string>("");
  const [test, setTest] = useState<TestResult | null>(null);
  const [saving, setSaving] = useState(false);
  const [tablesOpen, setTablesOpen] = useState(false);
  const [soundsOpen, setSoundsOpen] = useState(false);

  if (loading) return <div className="center">Cargando configuración...</div>;
  if (!config) return <div className="center">Error: {error}</div>;

  const zone = config.zones.find((z) => z.id === config.activeZone);
  const patch = (fn: (cfg: AppConfig) => AppConfig) => setConfig(fn(structuredClone(config)));

  const patchZone = (id: string, fn: (z: Zone) => Zone) =>
    patch((cfg) => ({
      ...cfg,
      zones: cfg.zones.map((z) => (z.id === id ? fn(structuredClone(z)) : z)),
    }));

  const addZone = () => {
    const base: Zone = {
      id: "nueva-zona",
      nombre: "Nueva zona",
      estacion: "ESTPACKXX",
      tables: {
        resultados: "PackoutResultadosNUEVO",
        errores: "PackoutErrNUEVO",
        usuarios: "PackoutUsrNUEVO",
        admin: "PackoutAdminNUEVO",
        recientes: "PackoutResViewNUEVO",
        itemImages: "PackoutItemsImgNUEVO",
      },
      mapics: {
        server: "",
        dsn: "",
        user: "",
        password: "",
        queryKit: "",
        queryInsert: "",
        queryDelete: "",
        queryBuffer: "",
      },
    };
    patch((cfg) => ({ ...cfg, zones: [...cfg.zones, base] }));
  };

  const removeZone = (id: string) =>
    patch((cfg) => {
      const zones = cfg.zones.filter((z) => z.id !== id);
      const activeZone = cfg.activeZone === id ? (zones[0]?.id ?? "") : cfg.activeZone;
      return { ...cfg, zones, activeZone };
    });

  const duplicateZone = (id: string) =>
    patch((cfg) => {
      const z = cfg.zones.find((x) => x.id === id);
      if (!z) return cfg;
      const copy: Zone = {
        ...structuredClone(z),
        id: `${z.id}-copia`,
        nombre: `${z.nombre} (copia)`,
      };
      return { ...cfg, zones: [...cfg.zones, copy] };
    });

  const handleSetActive = async (id: string) => {
    try {
      setConfig(await setActiveZone(id));
      setStatus("Zona activa cambiada");
    } catch (e) {
      setStatus(String(e));
    }
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await save();
      setStatus("Configuración guardada");
    } catch (e) {
      setStatus(`Error al guardar: ${e}`);
    } finally {
      setSaving(false);
    }
  };

  const handleExport = async () => {
    const path = await dialogSave({
      title: "Exportar configuración",
      defaultPath: "packout.config",
      filters: [{ name: "Config", extensions: ["config"] }],
    });
    if (!path) return;
    try {
      await exportConfig(path);
      setStatus(`Configuración exportada a: ${path}`);
    } catch (e) {
      setStatus(`Error al exportar: ${e}`);
    }
  };

  const handleImport = async () => {
    const path = await dialogOpen({
      title: "Importar configuración",
      multiple: false,
      directory: false,
      filters: [{ name: "Config", extensions: ["config"] }],
    });
    if (!path) return;
    try {
      const imported = await importConfig(path);
      setConfig(imported);
      await save();
      setStatus(`Configuración importada desde: ${path}`);
    } catch (e) {
      setStatus(`Error al importar: ${e}`);
    }
  };

  const handleTest = async () => {
    setStatus("Probando conexiones...");
    try {
      const res = await testZone();
      setTest(res);
      setStatus(
        `SQL: ${res.sql.ok ? "OK" : "FAIL"} · MAPICS: ${res.mapics.ok ? "OK" : "FAIL"}`,
      );
    } catch (e) {
      setStatus(String(e));
    }
  };

  return (
    <div className="settings">
      <header className="topbar">
        <h1>PACKOUT</h1>
        <div className="topbar-actions">
          {onBack && (
            <button onClick={onBack}>← Pantalla principal</button>
          )}
          <button onClick={handleTest}>Probar conexión</button>
          <button onClick={handleExport}>Exportar config</button>
          <button onClick={handleImport}>Importar config</button>
          <button onClick={handleSave} disabled={saving}>
            Guardar
          </button>
        </div>
      </header>

      {error && <p className="error">{error}</p>}
      {status && <p className="status">{status}</p>}

      <section className="card">
        <h2>Zonas</h2>
        <div className="zone-tabs">
          {config.zones.map((z) => (
            <button
              key={z.id}
              className={z.id === config.activeZone ? "zone-tab active" : "zone-tab"}
              onClick={() => handleSetActive(z.id)}
            >
              {z.nombre}
            </button>
          ))}
          <button className="zone-tab add" onClick={addZone}>
            + Nueva
          </button>
        </div>
        <div className="zone-actions">
          <button onClick={() => duplicateZone(config.activeZone)}>Duplicar zona activa</button>
          {config.zones.length > 1 && (
            <button className="danger" onClick={() => removeZone(config.activeZone)}>
              Eliminar zona activa
            </button>
          )}
        </div>
      </section>

      {zone && (
        <>
          <section className="card">
            <h2>Zona: {zone.nombre}</h2>
            <div className="grid">
              <Field
                label="ID (identificador único)"
                value={zone.id}
                onChange={(v) =>
                  patch((cfg) => {
                    const zones = cfg.zones.map((z) =>
                      z.id === zone.id ? { ...z, id: v } : z,
                    );
                    return {
                      ...cfg,
                      zones,
                      activeZone: zone.id === cfg.activeZone ? v : cfg.activeZone,
                    };
                  })
                }
              />
              <Field
                label="Nombre"
                value={zone.nombre}
                onChange={(v) => patchZone(zone.id, (z) => ({ ...z, nombre: v }))}
              />
              <Field
                label="Estación (ESTPACKXX)"
                value={zone.estacion}
                onChange={(v) => patchZone(zone.id, (z) => ({ ...z, estacion: v }))}
              />
            </div>
          </section>

          <section className="card">
            <h2>Tablas SQL</h2>
            <div className="settings-buttons">
              <button className="btn" onClick={() => setTablesOpen(true)}>
                Verificar tablas
              </button>
              <button className="btn" onClick={() => setSoundsOpen(true)}>
                Sonidos
              </button>
            </div>
            <div className="grid">
              <Field
                label="Resultados"
                value={zone.tables.resultados}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, tables: { ...z.tables, resultados: v } }))
                }
              />
              <Field
                label="Errores"
                value={zone.tables.errores}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, tables: { ...z.tables, errores: v } }))
                }
              />
              <Field
                label="Usuarios"
                value={zone.tables.usuarios}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, tables: { ...z.tables, usuarios: v } }))
                }
              />
              <Field
                label="Admin"
                value={zone.tables.admin}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, tables: { ...z.tables, admin: v } }))
                }
              />
              <Field
                label="Vista recientes"
                value={zone.tables.recientes}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, tables: { ...z.tables, recientes: v } }))
                }
              />
              <Field
                label="Imágenes de items"
                value={zone.tables.itemImages}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, tables: { ...z.tables, itemImages: v } }))
                }
              />
            </div>
          </section>

          <section className="card">
            <h2>MAPICS</h2>
            <div className="grid">
              <Field
                label="Servidor"
                value={zone.mapics.server}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, mapics: { ...z.mapics, server: v } }))
                }
              />
              <Field
                label="DSN"
                value={zone.mapics.dsn}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, mapics: { ...z.mapics, dsn: v } }))
                }
              />
              <Field
                label="Usuario"
                value={zone.mapics.user}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, mapics: { ...z.mapics, user: v } }))
                }
              />
              <Field
                label="Password"
                value={zone.mapics.password}
                onChange={(v) =>
                  patchZone(zone.id, (z) => ({ ...z, mapics: { ...z.mapics, password: v } }))
                }
              />
            </div>
            <Field
              label="Query Kit (usa {SERIE})"
              value={zone.mapics.queryKit}
              mono
              rows={5}
              onChange={(v) =>
                patchZone(zone.id, (z) => ({ ...z, mapics: { ...z.mapics, queryKit: v } }))
              }
            />
            <Field
              label="Query Insert (usa {SERIE}, {ESTACION})"
              value={zone.mapics.queryInsert}
              mono
              rows={3}
              onChange={(v) =>
                patchZone(zone.id, (z) => ({ ...z, mapics: { ...z.mapics, queryInsert: v } }))
              }
            />
            <Field
              label="Query Delete (usa {SERIE})"
              value={zone.mapics.queryDelete}
              mono
              rows={2}
              onChange={(v) =>
                patchZone(zone.id, (z) => ({ ...z, mapics: { ...z.mapics, queryDelete: v } }))
              }
            />
            <Field
              label="Query Buffer (usa {SERIE}, {LIMIT})"
              value={zone.mapics.queryBuffer}
              mono
              rows={3}
              onChange={(v) =>
                patchZone(zone.id, (z) => ({ ...z, mapics: { ...z.mapics, queryBuffer: v } }))
              }
            />
          </section>

          <section className="card">
            <h2>Conexión SQL (global)</h2>
            <div className="grid">
              <Field
                label="Servidor"
                value={config.sql.server}
                onChange={(v) =>
                  patch((cfg) => ({ ...cfg, sql: { ...cfg.sql, server: v } }))
                }
              />
              <Field
                label="Base de datos"
                value={config.sql.database}
                onChange={(v) =>
                  patch((cfg) => ({ ...cfg, sql: { ...cfg.sql, database: v } }))
                }
              />
              <Field
                label="Usuario"
                value={config.sql.user}
                onChange={(v) =>
                  patch((cfg) => ({ ...cfg, sql: { ...cfg.sql, user: v } }))
                }
              />
              <Field
                label="Password"
                value={config.sql.password}
                onChange={(v) =>
                  patch((cfg) => ({ ...cfg, sql: { ...cfg.sql, password: v } }))
                }
              />
              <Field
                label="Driver ODBC (ej: SQL Server, ODBC Driver 17 for SQL Server)"
                value={config.sql.driver}
                onChange={(v) =>
                  patch((cfg) => ({ ...cfg, sql: { ...cfg.sql, driver: v } }))
                }
              />
              <Field
                label="Buffer de kits (series a futuro a precargar)"
                value={String(config.bufferKits ?? 30)}
                onChange={(v) =>
                  patch((cfg) => ({ ...cfg, bufferKits: parseInt(v) || 0 }))
                }
              />
            </div>
          </section>
        </>
      )}

      {updater && (
        <section className="card">
          <h2>Actualizaciones</h2>
          {updater.state.phase === "checking" && <p className="muted">Buscando actualizaciones...</p>}
          {updater.state.phase === "idle" && (
            <p className="ok">Estás en la versión más reciente</p>
          )}
          {updater.state.phase === "available" && (
            <>
              <p className="ok">
                Nueva versión <strong>{updater.state.update.version}</strong> disponible (tienes{" "}
                {updater.state.update.current_version})
              </p>
              <div className="modal-actions">
                <button className="btn primary" onClick={updater.install}>
                  Descargar e instalar
                </button>
              </div>
            </>
          )}
          {updater.state.phase === "downloading" && (
            <p className="muted">
              Descargando... {(updater.state as { percent: number }).percent}%
            </p>
          )}
          {updater.state.phase === "installing" && <p className="muted">Instalando...</p>}
          {updater.state.phase === "done" && <p className="ok">Actualización instalada</p>}
          {updater.state.phase === "error" && (
            <>
              <p className="error">Error: {updater.state.message}</p>
            </>
          )}
          <div className="modal-actions">
            <button onClick={() => updater.check({ manual: true })} disabled={updater.state.phase === "checking"}>
              Buscar actualizaciones
            </button>
          </div>
        </section>
      )}

      {test && (
        <section className="card">
          <h2>Resultado de la prueba</h2>
          <p className={test.sql.ok ? "ok" : "error"}>
            SQL: {test.sql.ok ? "OK" : "FAIL"} — {test.sql.msg}
          </p>
          <p className={test.mapics.ok ? "ok" : "error"}>
            MAPICS: {test.mapics.ok ? "OK" : "FAIL"} — {test.mapics.msg}
          </p>
          <button onClick={reload}>Recargar config</button>
        </section>
      )}

      {tablesOpen && <TablesModal onClose={() => setTablesOpen(false)} />}

      {soundsOpen && config && (
        <SoundModal
          initial={{
            enabled: config.sound?.enabled ?? true,
            complete: config.sound?.complete ?? "",
            error: config.sound?.error ?? "",
          }}
          onClose={() => setSoundsOpen(false)}
          onSave={async (sound) => {
            const next = { ...config, sound };
            setConfig(next);
            await saveConfig(next);
          }}
        />
      )}
    </div>
  );
}