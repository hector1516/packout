import { useEffect, useState, type ReactNode } from "react";
import {
  sqlDeleteItemImage,
  sqlListItemImages,
  sqlSaveItemImage,
  type KitRow,
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