import { useEffect, useState } from "react";
import {
  mapicsQueryKit,
  sqlDeleteAdmin,
  sqlDeleteItemImage,
  sqlDeleteUsuario,
  sqlInsertAdmin,
  sqlInsertUsuario,
  sqlListAdmins,
  sqlListItemImages,
  sqlListUsuarios,
  sqlReportes,
  sqlSaveItemImage,
  sqlUpdateAdmin,
  sqlUpdateUsuario,
} from "../lib/packout";
import { useConfig } from "../hooks/useConfig";

type Tab = "reportes" | "fotos" | "usuarios" | "admins";

function todayStr() { return new Date().toISOString().slice(0,10); }
function toISO(d: Date) { return d.toISOString().slice(0,10); }

function exportCsv(rows: Record<string, unknown>[], filename: string) {
  if (rows.length===0) return;
  const cols = Object.keys(rows[0]);
  const esc = (v: unknown) => `"${String(v ?? "").replace(/"/g,'""')}"`;
  const csv = [cols.join(","), ...rows.map(r=>cols.map(c=>esc(r[c])).join(","))].join("\n");
  const blob = new Blob([csv], { type: "text/csv;charset=utf-8;" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a"); a.href=url; a.download=filename; a.click(); URL.revokeObjectURL(url);
}

// --- Reportes Tab
function ReportesTab() {
  const [tabla, setTabla] = useState("resultados");
  const [desde, setDesde] = useState("");
  const [hasta, setHasta] = useState("");
  const [resultado, setResultado] = useState("");
  const [busqueda, setBusqueda] = useState("");
  const [rows, setRows] = useState<Record<string,unknown>[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [cols, setCols] = useState<string[]>([]);
  const [visibleCols, setVisibleCols] = useState<Set<string>>(new Set());
  const [page, setPage] = useState(0);
  const limit = 50;

  const fetch = async (p=0) => {
    setLoading(true);
    try {
      const r = await sqlReportes({ tabla, desde: desde || undefined, hasta: hasta || undefined, resultado: resultado || undefined, busqueda: busqueda || undefined, limit, offset: p*limit });
      setRows(r.rows); setTotal(r.total);
      if (r.rows.length>0) {
        const c = Object.keys(r.rows[0]);
        setCols(c);
        if (visibleCols.size===0) setVisibleCols(new Set(c));
      } else { setCols([]); }
      setPage(p);
    } catch (e) { setRows([]); } finally { setLoading(false); }
  };

  useEffect(()=>{ fetch(0); }, [tabla]);

  const preset = (k: string) => {
    const now = new Date();
    if (k==="hoy") { const s=todayStr(); setDesde(s); setHasta(s); }
    else if (k==="semana") { const d=new Date(); d.setDate(d.getDate()-6); setDesde(toISO(d)); setHasta(todayStr()); }
    else if (k==="mes") { const d=new Date(now.getFullYear(), now.getMonth(), 1); setDesde(toISO(d)); setHasta(todayStr()); }
    else if (k==="anio") { const d=new Date(now.getFullYear(),0,1); setDesde(toISO(d)); setHasta(todayStr()); }
    else { setDesde(""); setHasta(""); }
  };

  return (
    <div className="admin-section">
      <div className="report-filters">
        <select value={tabla} onChange={e=>setTabla(e.target.value)}>
          <option value="resultados">Resultados</option>
          <option value="errores">Errores</option>
        </select>
        <div className="preset-row">
          <button className="btn subtle" onClick={()=>preset("hoy")}>Hoy</button>
          <button className="btn subtle" onClick={()=>preset("semana")}>Semana</button>
          <button className="btn subtle" onClick={()=>preset("mes")}>Mes</button>
          <button className="btn subtle" onClick={()=>preset("anio")}>Año</button>
          <button className="btn subtle" onClick={()=>preset("todo")}>Todo</button>
        </div>
        <input type="date" value={desde} onChange={e=>setDesde(e.target.value)} />
        <span className="muted">→</span>
        <input type="date" value={hasta} onChange={e=>setHasta(e.target.value)} />
        <select value={resultado} onChange={e=>setResultado(e.target.value)}>
          <option value="">Todos</option><option value="APROBADO">APROBADO</option><option value="PENDIENTE">PENDIENTE</option><option value="MANUAL">MANUAL</option>
        </select>
        <input placeholder="Buscar en todo..." value={busqueda} onChange={e=>setBusqueda(e.target.value)} style={{minWidth:160}} />
        <button className="btn primary" onClick={()=>fetch(0)} disabled={loading}>{loading?"Cargando...":"Consultar"}</button>
        <button className="btn" onClick={()=>exportCsv(rows, `reporte-${tabla}-${todayStr()}.csv`)} disabled={rows.length===0}>⬇ Excel</button>
      </div>

      {cols.length>0 && (
        <div className="col-toggles">
          {cols.map(c=>(
            <label key={c} className="col-toggle">
              <input type="checkbox" checked={visibleCols.has(c)} onChange={e=>{
                const n=new Set(visibleCols); if(e.target.checked) n.add(c); else n.delete(c); setVisibleCols(n);
              }} /> {c}
            </label>
          ))}
        </div>
      )}

      <div className="table-wrap">
        <table className="table admin-table">
          <thead><tr>{cols.filter(c=>visibleCols.has(c)).map(c=><th key={c}>{c}</th>)}<th></th></tr></thead>
          <tbody>
            {rows.map((r,i)=>(
              <tr key={i}>
                {cols.filter(c=>visibleCols.has(c)).map(c=><td key={c} title={String(r[c]??"")}>{String(r[c]??"").slice(0,80)}</td>)}
                <td><button className="btn subtle" onClick={async()=>{
                  const serie = String(r["Serie"] ?? r["serie"] ?? "");
                  if(serie){ try{ const k=await mapicsQueryKit(serie); alert(`MAPICS ${serie}: ${k.count} filas\n${JSON.stringify(k.rows.slice(0,1),null,2).slice(0,400)}`)}catch{ alert("Sin datos MAPICS")} }
                }}>MAPICS</button></td>
              </tr>
            ))}
            {rows.length===0 && <tr><td colSpan={cols.length+1} className="muted">{loading?"Cargando...":"Sin datos — ajusta filtros y consulta"}</td></tr>}
          </tbody>
        </table>
      </div>
      <div className="pagination">
        <button className="btn subtle" disabled={page===0} onClick={()=>fetch(page-1)}>‹ Anterior</button>
        <span className="muted">Página {page+1} · {total} registros</span>
        <button className="btn subtle" disabled={(page+1)*limit >= total} onClick={()=>fetch(page+1)}>Siguiente ›</button>
      </div>
    </div>
  );
}

// --- Fotos Tab (visual)
function FotosTab() {
  const [items, setItems] = useState<{Item:string}[]>([]);
  const [selected, setSelected] = useState<string>("");
  const [preview, setPreview] = useState<string>("");
  const [dragOver, setDragOver] = useState(false);

  const reload = async()=>{ try{ const r=await sqlListItemImages(); setItems(r.rows.map((x:any)=>({Item: x.Item ?? x.item})));}catch{} };
  useEffect(()=>{ reload(); }, []);

  // file input + drag & drop for preview
  const onFile = (file: File)=>{
    const reader = new FileReader();
    reader.onload = ()=> setPreview(reader.result as string);
    reader.readAsDataURL(file);
  };

  return (
    <div className="admin-section">
      <div className="photo-admin-grid">
        <div className="photo-list">
          <h3>Items con foto ({items.length})</h3>
          <div className="chip-list">
            {items.map(it=>(
              <button key={it.Item} className={`chip ${selected===it.Item?"active":""}`} onClick={()=>setSelected(it.Item)}>{it.Item}</button>
            ))}
            {items.length===0 && <span className="muted">Sin fotos aún</span>}
          </div>
          <button className="btn" onClick={reload}>Recargar</button>
        </div>

        <div className="photo-editor">
          <h3>Agregar / editar foto</h3>
          <input placeholder="Código del item (ej. CABLE-10)" value={selected} onChange={e=>setSelected(e.target.value.toUpperCase())} className="mono" />
          <div
            className={`drop-zone ${dragOver?"over":""}`}
            onDragOver={e=>{e.preventDefault(); setDragOver(true);}}
            onDragLeave={()=>setDragOver(false)}
            onDrop={e=>{ e.preventDefault(); setDragOver(false); const f=e.dataTransfer.files[0]; if(f) onFile(f); }}
          >
            {preview ? <img src={preview} alt="preview" className="drop-preview" /> : <span className="muted">Arrastra imagen aquí o usa el selector</span>}
          </div>
          <input type="file" accept="image/*" onChange={e=>{ const f=e.target.files?.[0]; if(f) onFile(f); }} />
          <div className="modal-actions">
            <button className="btn primary" disabled={!selected || !preview} onClick={async()=>{
              await sqlSaveItemImage(selected.trim().toUpperCase(), preview);
              setPreview(""); setSelected(""); reload();
            }}>Guardar foto</button>
            <button className="btn danger" disabled={!selected} onClick={async()=>{
              await sqlDeleteItemImage(selected.trim().toUpperCase()); reload();
            }}>Eliminar</button>
          </div>
          {selected && <p className="muted">Se guarda en <code>PackoutItemsImgIMX</code> como base64, se ve en la cuadrícula de Operación.</p>}
        </div>
      </div>
    </div>
  );
}

// --- Usuarios / Admins Tab (visual editable)
function CrudTab({ kind }: { kind: "usuario"|"admin" }) {
  const isUser = kind==="usuario";
  const [rows, setRows] = useState<Record<string,unknown>[]>([]);
  const [no, setNo] = useState(""); const [nombre, setNombre] = useState("");
  const [editing, setEditing] = useState<string | null>(null);
  const [search, setSearch] = useState("");

  const load = async()=>{
    const r = isUser ? await sqlListUsuarios() : await sqlListAdmins();
    setRows(r.rows);
  };
  useEffect(()=>{ load(); }, []);

  const filtered = rows.filter(r=>{
    if(!search) return true;
    const s=search.toLowerCase();
    return String(r["No"]??"").toLowerCase().includes(s) || String(r["Nombre"]??"").toLowerCase().includes(s);
  });

  return (
    <div className="admin-section">
      <div className="crud-header">
        <input placeholder="Buscar por No o Nombre..." value={search} onChange={e=>setSearch(e.target.value)} style={{minWidth:220}} />
        <span className="muted">{filtered.length} registros</span>
      </div>

      <div className="crud-form">
        <input placeholder="No (gafete)" value={no} onChange={e=>setNo(e.target.value)} />
        <input placeholder="Nombre" value={nombre} onChange={e=>setNombre(e.target.value)} />
        <button className="btn primary" onClick={async()=>{
          if(!no) return;
          if(editing){ isUser ? await sqlUpdateUsuario(no, nombre) : await sqlUpdateAdmin(no, nombre); setEditing(null); }
          else { isUser ? await sqlInsertUsuario(no, nombre) : await sqlInsertAdmin(no, nombre); }
          setNo(""); setNombre(""); load();
        }}>{editing ? "Actualizar" : "Agregar"}</button>
        {editing && <button className="btn" onClick={()=>{setEditing(null); setNo(""); setNombre("");}}>Cancelar</button>}
      </div>

      <div className="table-wrap">
        <table className="table admin-table">
          <thead><tr><th>No</th><th>Nombre</th><th style={{width:160}}>Acciones</th></tr></thead>
          <tbody>
            {filtered.map((r,i)=>(
              <tr key={i} className={editing===String(r["No"])?"row-editing":""}>
                <td><span className="item-code">{String(r["No"]??"")}</span></td>
                <td>{String(r["Nombre"]??"")}</td>
                <td>
                  <button className="btn subtle" onClick={()=>{ setEditing(String(r["No"])); setNo(String(r["No"])); setNombre(String(r["Nombre"]??"")); }}>✏️ Editar</button>
                  <button className="btn subtle danger" onClick={async()=>{
                    if(confirm(`Eliminar ${r["No"]}?`)){
                      isUser ? await sqlDeleteUsuario(String(r["No"])) : await sqlDeleteAdmin(String(r["No"]));
                      load();
                    }
                  }}>🗑️</button>
                </td>
              </tr>
            ))}
            {filtered.length===0 && <tr><td colSpan={3} className="muted">Sin registros</td></tr>}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export function AdminPanel({ onOpenSettings }: { onOpenSettings: () => void }) {
  const [tab, setTab] = useState<Tab>("reportes");
  const { config } = useConfig();
  const zone = config?.zones.find((z) => z.id === config.activeZone)?.nombre ?? "—";

  return (
    <div className="screen">
      <header className="topbar">
        <div className="brand">
          <img src="/hussmann.png" alt="Hussmann" className="brand-logo" draggable={false} />
          <span className="brand-title">PACKOUT</span>
          <span className="muted">Administración · {zone}</span>
        </div>
        <div className="topbar-actions">
          <button className="btn" onClick={onOpenSettings}>Configuración</button>
        </div>
      </header>

      <div className="admin-tabs">
        <button className={`admin-tab ${tab==="reportes"?"active":""}`} onClick={()=>setTab("reportes")}>📊 Reportes</button>
        <button className={`admin-tab ${tab==="fotos"?"active":""}`} onClick={()=>setTab("fotos")}>🖼️ Fotos</button>
        <button className={`admin-tab ${tab==="usuarios"?"active":""}`} onClick={()=>setTab("usuarios")}>👥 Colaboradores</button>
        <button className={`admin-tab ${tab==="admins"?"active":""}`} onClick={()=>setTab("admins")}>🔑 Admins</button>
      </div>

      <div className="admin-content">
        {tab==="reportes" && <ReportesTab />}
        {tab==="fotos" && <FotosTab />}
        {tab==="usuarios" && <CrudTab kind="usuario" />}
        {tab==="admins" && <CrudTab kind="admin" />}
      </div>
    </div>
  );
}
