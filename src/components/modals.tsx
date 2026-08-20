import { useEffect, useState, type ReactNode } from "react";
import {
  sqlCheckTables,
  sqlCreateTables,
  sqlDeleteItemImage,
  sqlListItemImages,
  sqlSaveItemImage,
  type KitRow,
  type PendingOp,
  type TableCheck,
} from "../lib/packout";
export function Modal({
  title,
  onClose,
  children,
}: {
  title: string;
  onClose: () => void;
  children: ReactNode;
}) {
  return (
    <div className="modal-backdrop" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <span>{title}</span>
          <button className="btn subtle" onClick={onClose}>
            ✕
          </button>
        </div>
        <div className="modal-body">{children}</div>
      </div>
    </div>
  );
}

export function Field({
  label,
  children,
}: {
  label: string;
  children: ReactNode;
}) {
  return (
    <label className="field">
      <span>{label}</span>
      {children}
    </label>
  );
}

export function OperatorModal({
  title,
  inputLabel,
  onOk,
  onClose,
  errorMsg,
}: {
  title: string;
  inputLabel: string;
  onOk: (no: string) => void | Promise<boolean | void>;
  onClose: () => void;
  errorMsg?: string;
}) {
  const [no, setNo] = useState("");
  const [msg, setMsg] = useState(errorMsg ?? "");

  const submit = async () => {
    if (!no.trim()) return;
    const res = await onOk(no.trim().toUpperCase());
    if (res === false) setMsg("No registrado en la base de datos");
  };

  return (
    <Modal title={title} onClose={onClose}>
      <Field label={inputLabel}>
        <input
          autoFocus
          value={no}
          onChange={(e) => {
            setNo(e.target.value);
            setMsg("");
          }}
          onKeyDown={(e) => e.key === "Enter" && submit()}
        />
      </Field>
      {msg && <p className="error-text">{msg}</p>}
      <div className="modal-actions">
        <button className="btn primary" onClick={submit}>
          Aceptar
        </button>
        <button className="btn" onClick={onClose}>
          Cancelar
        </button>
      </div>
    </Modal>
  );
}

export function PendingModal({
  onSave,
  onClose,
}: {
  onSave: (admin: string, comentario: string) => void;
  onClose: () => void;
}) {
  const [admin, setAdmin] = useState("");
  const [comentario, setComentario] = useState("");

  const submit = () => {
    if (!admin.trim()) return;
    onSave(admin.trim().toUpperCase(), comentario.trim());
  };

  return (
    <Modal title="Registrar pendiente" onClose={onClose}>
      <p className="muted">El serial actual quedará como PENDIENTE en el historial.</p>
      <Field label="Gafete / admin">
        <input
          autoFocus
          value={admin}
          onChange={(e) => setAdmin(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && submit()}
        />
      </Field>
      <Field label="Comentario">
        <input value={comentario} onChange={(e) => setComentario(e.target.value)} />
      </Field>
      <div className="modal-actions">
        <button className="btn primary" onClick={submit}>
          Guardar
        </button>
        <button className="btn" onClick={onClose}>
          Cancelar
        </button>
      </div>
    </Modal>
  );
}

export function ReprintModal({
  onSave,
  onClose,
  listRecientes,
}: {
  onSave: (serie: string) => void;
  onClose: () => void;
  listRecientes: (top?: number) => Promise<KitRow[]>;
}) {
  const [series, setSeries] = useState<KitRow[]>([]);
  const [serie, setSerie] = useState("");
  const [msg, setMsg] = useState("");

  useEffect(() => {
    listRecientes(20)
      .then((rows) => {
        setSeries(rows);
        if (rows.length > 0) setSerie(rows[0]["SERIE"] ?? rows[0]["serie"] ?? "");
      })
      .catch(() => setMsg("No se pudieron cargar las series recientes"));
  }, [listRecientes]);

  const submit = () => {
    const s = serie.replace(/[()]/g, "").trim().toUpperCase();
    if (!s) return;
    onSave(s);
  };

  return (
    <Modal title="Reimprimir" onClose={onClose}>
      <p className="muted">Borra y vuelve a insertar el kit en MAPICS (FESRLKIT).</p>
      <Field label="Últimas series aprobadas">
        <select value={serie} onChange={(e) => setSerie(e.target.value)}>
          {series.map((r, i) => (
            <option key={i} value={r["SERIE"] ?? r["serie"] ?? ""}>
              {r["SERIE"] ?? r["serie"] ?? "—"}
            </option>
          ))}
        </select>
      </Field>
      <Field label="O escribe un serial manual">
        <input
          value={serie}
          onChange={(e) => setSerie(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && submit()}
        />
      </Field>
      {msg && <p className="muted">{msg}</p>}
      <div className="modal-actions">
        <button className="btn primary" onClick={submit}>
          Reimprimir
        </button>
        <button className="btn" onClick={onClose}>
          Cancelar
        </button>
      </div>
    </Modal>
  );
}

export function ImagesModal({
  onClose,
  onChanged,
}: {
  onClose: () => void;
  onChanged: () => void;
}) {
  const [item, setItem] = useState("");
  const [preview, setPreview] = useState<string | null>(null);
  const [itemsWithImg, setItemsWithImg] = useState<string[]>([]);
  const [msg, setMsg] = useState("");

  const refresh = async () => {
    try {
      const res = await sqlListItemImages();
      setItemsWithImg(res.rows.map((r) => r["Item"] ?? r["item"] ?? ""));
    } catch (e) {
      setMsg(String(e));
    }
  };

  useEffect(() => {
    refresh();
  }, []);

  const onFile = (f: File | undefined) => {
    if (!f) return;
    const reader = new FileReader();
    reader.onload = () => {
      setPreview(reader.result as string);
      setMsg("");
    };
    reader.readAsDataURL(f);
  };

  const save = async () => {
    if (!item.trim() || !preview) return;
    try {
      await sqlSaveItemImage(item.trim().toUpperCase(), preview);
      setMsg("Guardado");
      setPreview(null);
      onChanged();
      refresh();
    } catch (e) {
      setMsg(String(e));
    }
  };

  const del = async (it: string) => {
    try {
      await sqlDeleteItemImage(it);
      setMsg(`Eliminada imagen de ${it}`);
      onChanged();
      refresh();
    } catch (e) {
      setMsg(String(e));
    }
  };

  return (
    <Modal title="Imágenes de items" onClose={onClose}>
      <div className="img-manager">
        <Field label="Item (código)">
          <input
            value={item}
            onChange={(e) => setItem(e.target.value)}
            placeholder="ej: CJAJUL8"
          />
        </Field>
        <Field label="Imagen">
          <input type="file" accept="image/*" onChange={(e) => onFile(e.target.files?.[0])} />
        </Field>
        {preview && (
          <div className="img-preview">
            <img src={preview} alt="preview" />
          </div>
        )}
        <button className="btn primary" onClick={save} disabled={!preview || !item.trim()}>
          Guardar imagen
        </button>
        {msg && <p className="muted">{msg}</p>}
        <ul className="img-list">
          {itemsWithImg.map((it) => (
            <li key={it} className="img-list-row">
              <span>{it}</span>
              <button className="btn danger" onClick={() => del(it)}>
                Eliminar
              </button>
            </li>
          ))}
          {itemsWithImg.length === 0 && <li className="muted">sin imágenes guardadas</li>}
        </ul>
      </div>
    </Modal>
  );
}

export function BufferModal({
  onClose,
  onSync,
  sqlOnline,
  mapicsOnline,
  cola,
  kits,
  procesadas,
  syncing,
}: {
  onClose: () => void;
  onSync: () => void;
  sqlOnline: boolean;
  mapicsOnline: boolean;
  cola: PendingOp[];
  kits: { serie: string; pedido: string; items: number; image: boolean }[];
  procesadas: string[];
  syncing: boolean;
}) {
  const both = sqlOnline && mapicsOnline;

  return (
    <Modal title="Buffer de sincronización" onClose={onClose}>
      <div className="buffer">
        <div className="buffer-status">
          <span className={sqlOnline ? "dot ok" : "dot bad"}>●</span> SQL:{" "}
          {sqlOnline ? "EN LÍNEA" : "SIN CONEXIÓN"}
          <span className={mapicsOnline ? "dot ok" : "dot bad"}>●</span> MAPICS:{" "}
          {mapicsOnline ? "EN LÍNEA" : "SIN CONEXIÓN"}
        </div>
        {!both && (
          <p className="error-text">
            La sincronización requiere que SQL y MAPICS estén en línea al mismo tiempo.
          </p>
        )}
        <table className="table">
          <thead>
            <tr>
              <th>Serie</th>
              <th>MAPICS</th>
              <th>SQL</th>
              <th>Fecha</th>
            </tr>
          </thead>
          <tbody>
            {cola.map((op) => (
              <tr key={op.serie}>
                <td>{op.serie}</td>
                <td>{op.mapicsOk ? "✓ enviado" : "pendiente"}</td>
                <td>{op.sqlOk ? "✓ enviado" : "pendiente"}</td>
                <td>{op.fecha}</td>
              </tr>
            ))}
            {cola.length === 0 && (
              <tr>
                <td colSpan={4} className="muted">
                  sin pendientes — buffer vacío
                </td>
              </tr>
            )}
          </tbody>
        </table>
        <div className="buffer-meta">
          <span className="chip">Kits en buffer: {kits.length}</span>
          <span className="chip">Procesadas: {procesadas.length}</span>
          <span className="chip">Pendientes: {cola.length}</span>
        </div>
        <div className="modal-actions">
          <button className="btn primary" onClick={onSync} disabled={!both || syncing}>
            {syncing ? "Sincronizando..." : "Sincronizar ahora"}
          </button>
          <button className="btn" onClick={onClose}>
            Cerrar
          </button>
        </div>
      </div>
    </Modal>
  );
}

export function TablesModal({
  onClose,
}: {
  onClose: () => void;
}) {
  const [tables, setTables] = useState<TableCheck[]>([]);
  const [checking, setChecking] = useState(false);
  const [msg, setMsg] = useState("");

  const check = async () => {
    setChecking(true);
    setMsg("");
    try {
      const res = await sqlCheckTables();
      setTables(res.tables);
      const bad = res.tables.filter((t) => !t.ok);
      if (bad.length === 0) setMsg("Todas las tablas están correctas");
      else setMsg(`${bad.length} tabla(s) con problemas`);
    } catch (e) {
      setMsg(String(e));
    } finally {
      setChecking(false);
    }
  };

  const create = async () => {
    setChecking(true);
    setMsg("");
    try {
      const res = await sqlCreateTables();
      setTables(res.tables);
      const bad = res.tables.filter((t) => !t.ok);
      if (bad.length === 0) setMsg("Tablas creadas/verificadas correctamente");
      else setMsg(`${bad.length} tabla(s) siguen con problemas`);
    } catch (e) {
      setMsg(String(e));
    } finally {
      setChecking(false);
    }
  };

  useEffect(() => {
    check();
  }, []);

  return (
    <Modal title="Verificar tablas SQL" onClose={onClose}>
      <div className="tables-check">
        <p className="muted">
          Escanea la base de datos para verificar que existan las tablas con sus campos
          correctos. Si falta una tabla o campo, se creará.
        </p>
        <table className="table">
          <thead>
            <tr>
              <th>Tabla</th>
              <th>Estado</th>
              <th>Faltantes</th>
            </tr>
          </thead>
          <tbody>
            {tables.map((t) => (
              <tr key={t.table}>
                <td>
                  {t.isView ? "vista " : ""}
                  {t.table}
                </td>
                <td>
                  {t.exists ? (
                    <span className={t.ok ? "ok" : "error-text"}>
                      {t.ok ? "✓ correcta" : "existe, faltan campos"}
                    </span>
                  ) : (
                    <span className="error-text">no existe</span>
                  )}
                </td>
                <td>
                  {t.missing.length > 0 ? (
                    <span className="error-text">{t.missing.join(", ")}</span>
                  ) : (
                    "—"
                  )}
                </td>
              </tr>
            ))}
            {tables.length === 0 && (
              <tr>
                <td colSpan={3} className="muted">
                  {checking ? "Verificando..." : "Sin resultados"}
                </td>
              </tr>
            )}
          </tbody>
        </table>
        {msg && <p className={msg.includes("problemas") ? "error-text" : "ok"}>{msg}</p>}
        <div className="modal-actions">
          <button className="btn primary" onClick={create} disabled={checking}>
            {checking ? "Procesando..." : "Crear / reparar tablas"}
          </button>
          <button className="btn" onClick={check} disabled={checking}>
            Re-verificar
          </button>
          <button className="btn" onClick={onClose}>
            Cerrar
          </button>
        </div>
      </div>
    </Modal>
  );
}