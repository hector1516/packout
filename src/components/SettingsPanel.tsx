import { useEffect, useState } from "react";
import { save as dialogSave, open as dialogOpen } from "@tauri-apps/plugin-dialog";
import { sqlCleanup } from "../lib/packout";
import {
  setActiveZone,
  testZone,
  exportConfig,
  importConfig,
  saveConfig,
  sqlScanRed,
  sqlListDatabases,
  mapicsTest,
  restoreMapicsDefaults,
  sqlGuardStatus,
  sqlGuardReset,
  type GuardStatus,
  type MapicsTestResult,
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
  const [scanBase, setScanBase] = useState("");
  const [scanning, setScanning] = useState(false);
  const [servers, setServers] = useState<string[]>([]);
  const [listing, setListing] = useState(false);
  const [databases, setDatabases] = useState<string[]>([]);
  const [mapicsTesting, setMapicsTesting] = useState(false);
  const [mapicsRes, setMapicsRes] = useState<MapicsTestResult | null>(null);
  const [guardStatus, setGuardStatus] = useState<GuardStatus | null>(null);

  useEffect(() => {
    sqlGuardStatus().then(setGuardStatus).catch(() => {});
  }, []);

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
      sqlGuardStatus().then(setGuardStatus).catch(() => {});
      setStatus(
        `SQL: ${res.sql.ok ? "OK" : "FAIL"} · MAPICS: ${res.mapics.ok ? "OK" : "FAIL"}`,
      );
    } catch (e) {
      setStatus(String(e));
    }
  };

  const handleScan = async () => {
    if (!config) return;
    const base = scanBase.trim() || config.sql.server;
    if (!base.trim()) {
      setStatus("Escribe la IP base de la red (ej. 10.96.16.114)");
      return;
    }
    setScanning(true);
    setStatus(`Buscando servidores SQL en la red de ${base}...`);
    try {
      const res = await sqlScanRed(base);
      setServers(res.servers);
      setStatus(
        res.servers.length > 0
          ? `${res.servers.length} servidor(es) con SQL encontrados`
          : "No se encontró ningún servidor SQL en esa red",
      );
    } catch (e) {
      setStatus(`Error al buscar: ${e}`);
    } finally {
      setScanning(false);
    }
  };

  const handleListDatabases = async () => {
    if (!config) return;
    setListing(true);
    setStatus(`Pidiendo lista de bases a ${config.sql.server}...`);
    try {
      const res = await sqlListDatabases(
        config.sql.server,
        config.sql.user,
        config.sql.password,
      );
      setDatabases(res.databases);
      setStatus(
        res.databases.length > 0
          ? `${res.databases.length} base(s) encontradas`
          : "Conectó pero no devolvió bases",
      );
    } catch (e) {
      setStatus(`Error al listar bases: ${e}`);
    } finally {
      setListing(false);
    }
  };

  const handleTestMapics = async () => {
    if (!zone) return;
    setMapicsTesting(true);
    setStatus("Probando conexión MAPICS...");
    try {
      const res = await mapicsTest({
        server: zone.mapics.server,
        dsn: zone.mapics.dsn,
        user: zone.mapics.user,
        password: zone.mapics.password,
      });
      setMapicsRes(res);
      setStatus(res.ok ? "MAPICS conectado" : `MAPICS falló: ${res.msg}`);
    } catch (e) {
      setStatus(`Error al probar MAPICS: ${e}`);
    } finally {
      setMapicsTesting(false);
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
            <button className="btn" onClick={() => setTablesOpen(true)}>
              Verificar tablas
            </button>
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
            <div className="modal-actions">
              <button className="btn" onClick={handleTestMapics} disabled={mapicsTesting}>
                {mapicsTesting ? "Probando..." : "🔌 Probar conexión MAPICS"}
              </button>
              <button
                className="btn"
                onClick={async () => {
                  try {
                    const next = await restoreMapicsDefaults(zone.id);
                    setConfig(next);
                    setStatus("Queries MAPICS restauradas a fábrica (LINEPR = 8)");
                  } catch (e) {
                    setStatus(String(e));
                  }
                }}
              >
                ♻️ Restaurar queries de fábrica
              </button>
            </div>
            {mapicsRes && (
              <>
                <p className={mapicsRes.ok ? "ok" : "error"}>
                  {mapicsRes.ok ? "OK" : "FAIL"} — {mapicsRes.msg}
                  <br />
                  <span className="mono">Query: {mapicsRes.query}</span>
                </p>
                {mapicsRes.ok && mapicsRes.rows.length > 0 && (
                  <table className="admin-table">
                    <thead>
                      <tr>
                        {mapicsRes.columns.map((c) => (
                          <th key={c}>{c}</th>
                        ))}
                      </tr>
                    </thead>
                    <tbody>
                      {mapicsRes.rows.map((r, i) => (
                        <tr key={i}>
                          {mapicsRes.columns.map((c) => (
                            <td key={c} className="mono">
                              {r[c] ?? ""}
                            </td>
                          ))}
                        </tr>
                      ))}
                    </tbody>
                  </table>
                )}
              </>
            )}
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
            <h3>Buscar servidor en la red</h3>
            <div className="guard-box">
              <p className="muted">
                Protección anti-bloqueo: como máximo{" "}
                <strong>{guardStatus?.maxAttempts ?? 3}</strong> intentos de inicio de
                sesión seguidos. Si fallan, la app deja de intentar para no bloquear
                la cuenta de SQL Server. El estado se comprueba con ping, sin
                autenticarse.
              </p>
              {guardStatus && (
                <p className={guardStatus.blocked ? "error" : "ok"}>
                  {guardStatus.blocked
                    ? `PAUSADO — ${guardStatus.blockReason}`
                    : `Normal · fallos de autenticación ${guardStatus.authFailures}/${guardStatus.maxAttempts} · fallos de red ${guardStatus.netFailures}`}
                </p>
              )}
              {guardStatus?.blocked && (
                <div className="modal-actions">
                  <button
                    className="btn primary"
                    onClick={async () => {
                      setGuardStatus(await sqlGuardReset());
                      setStatus("Intentos reiniciados. Vuelve a probar la conexión.");
                    }}
                  >
                    ♻️ Reintentar ahora
                  </button>
                </div>
              )}
            </div>
            <p className="muted">
              Escribe la IP (ej. 10.96.16.114) y busca equipos con SQL Server
              (puerto 1433) en esa red. Luego elige uno de la lista.
            </p>
            <div className="grid">
              <Field
                label="IP base de la red (vacío = usar servidor actual)"
                value={scanBase}
                onChange={setScanBase}
              />
            </div>
            <div className="modal-actions">
              <button className="btn" onClick={handleScan} disabled={scanning}>
                {scanning ? "Buscando..." : "🔍 Buscar servidores"}
              </button>
            </div>
            {servers.length > 0 && (
              <label className="field">
                <span>Servidores encontrados — elige uno para usarlo</span>
                <select
                  value={servers.includes(config.sql.server) ? config.sql.server : ""}
                  onChange={(e) => {
                    const v = e.currentTarget.value;
                    if (v) {
                      patch((cfg) => ({ ...cfg, sql: { ...cfg.sql, server: v } }));
                      setStatus(`Servidor cambiado a ${v}. No olvides Guardar.`);
                    }
                  }}
                >
                  <option value="">— Seleccionar —</option>
                  {servers.map((s) => (
                    <option key={s} value={s}>
                      {s}
                    </option>
                  ))}
                </select>
              </label>
            )}
            <h3>Elegir base de datos</h3>
            <p className="muted">
              Con el servidor, usuario y password de arriba, pide la lista de
              bases y elige una.
            </p>
            <div className="modal-actions">
              <button className="btn" onClick={handleListDatabases} disabled={listing}>
                {listing ? "Pidiendo lista..." : "📋 Listar bases de datos"}
              </button>
            </div>
            {databases.length > 0 && (
              <label className="field">
                <span>Bases encontradas — elige una para usarla</span>
                <select
                  value={databases.includes(config.sql.database) ? config.sql.database : ""}
                  onChange={(e) => {
                    const v = e.currentTarget.value;
                    if (v) {
                      patch((cfg) => ({ ...cfg, sql: { ...cfg.sql, database: v } }));
                      setStatus(`Base de datos cambiada a ${v}. No olvides Guardar.`);
                    }
                  }}
                >
                  <option value="">— Seleccionar —</option>
                  {databases.map((d) => (
                    <option key={d} value={d}>
                      {d}
                    </option>
                  ))}
                </select>
              </label>
            )}
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

      {config && (
        <section className="card">
          <h2>Sonidos</h2>
          <button className="btn" onClick={() => setSoundsOpen(true)}>
            Configurar sonidos
          </button>
        </section>
      )}

      {config && (
        <section className="card">
          <h2>Modo de la aplicación</h2>
          <p className="muted">Cambia entre Operación (escaneo) y Administración (reportes, fotos, usuarios). Se guarda y se recuerda al abrir.</p>
          <div className="mode-toggle">
            <button
              className={`btn ${config.modo !== "administracion" ? "primary" : ""}`}
              onClick={async () => {
                const next = { ...config, modo: "operacion", modoElegido: true };
                setConfig(next);
                await saveConfig(next);
                setStatus("Modo cambiado a Operación");
              }}
            >
              ⚙️ Operación
            </button>
            <button
              className={`btn ${config.modo === "administracion" ? "primary" : ""}`}
              onClick={async () => {
                const next = { ...config, modo: "administracion", modoElegido: true };
                setConfig(next);
                await saveConfig(next);
                setStatus("Modo cambiado a Administración");
              }}
            >
              📊 Administración
            </button>
          </div>
          <p className="muted">Actual: <strong>{config.modo === "administracion" ? "Administración" : "Operación"}</strong></p>
        </section>
      )}

      <section className="card">
        <h2>Mantenimiento</h2>
        <p className="muted">Conserva solo el último año. Borra registros de hace más de 365 días.</p>
        <button
          className="btn"
          onClick={async () => {
            setStatus("Limpiando registros antiguos...");
            try {
              const r = await sqlCleanup(365);
              setStatus(`Limpieza completada: ${r.deleted} registros eliminados (previo a ${r.cutoff})`);
              localStorage.setItem("packout_last_cleanup", String(Date.now()));
            } catch (e) {
              setStatus(String(e));
            }
          }}
        >
          Limpiar registros antiguos (&gt;1 año)
        </button>
        <p className="muted">Automático cada semana al abrir la app.</p>
      </section>

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