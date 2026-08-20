# Decisiones importantes

## DEC-001 — SQL Server por tiberius (nativo), ODBC solo para MAPICS

Estado: ACTIVA

Decisión:
- SQL Server se conecta con la crate `tiberius` (protocolo TDS nativo, equivalente al `SqlClient` del VB.NET), sin ODBC.
- MAPICS (DB2) sigue conectándose con la crate `odbc` vía DSN.

Motivo:
- En el VB.NET original, SQL usaba `System.Data.SqlClient` (nativo) y MAPICS `System.Data.Odbc` (DSN). Para no requerir drivers ODBC de SQL Server en la PC de producción, se replicó esa separación.

Alternativas descartadas:
- Usar ODBC para ambas conexiones (original del port, requería driver ODBC de SQL en producción).
- Usar otra crate de driver nativo.

Consecuencia:
- `sql.rs` usa tiberius + tokio; `mapics.rs` usa odbc. Ver `ARCHITECTURE.md`.
- LocalDB (named pipes) ya no sirve para probar SQL; se requiere un SQL Server con TCP (ver `ISSUES.md`).

## DEC-002 — Actualización automática vía GitHub Releases (tauri-plugin-updater)

Estado: ACTIVA

Decisión:
- Se reemplaza ClickOnce (VB) por `tauri-plugin-updater` consumiendo `https://github.com/hector1516/packout/releases/latest/download/latest.json`.
- Builds release firmados con llaves propias del updater; instalador `NSIS`/`MSI` en modo pasivo.

Motivo:
- ClickOnce no aplica a Tauri; GitHub Releases permite auto-actualización sin infraestructura extra.

Alternativas descartadas:
- Despliegue manual solo con exe (sin actualización).
- ClickOnce.

Consecuencia:
- Se requiere instalador (NSIS/MSI), no solo exe suelto, para que el updater funcione.
- Las llaves de firma son obligatorias al hacer build release. Ver `SECURITY.md` y `INTEGRATIONS.md`.

## DEC-003 — Config persistente en JSON editable por zona

Estado: ACTIVA

Decisión:
- La configuración (SQL, MAPICS, tablas, estación, queries) vive en `packout.config.json` en `%APPDATA%/com.packout.app/`, con export/import vía UI (SettingsPanel).

Motivo:
- Permite desplegar a distintas PCs/series sin recompilar; el VB original tenía la configuración embebida.

Alternativas descartadas:
- Hardcodear credenciales/queries en código.

Consecuencia:
- Cuidado con los secretos en ese JSON (ver `SECURITY.md`). La config de trabajo NO debe subirse al repo.

## DEC-004 — Branding con colores y logos de Hussmann/ECCSA

Estado: ACTIVA

Decisión:
- Se extrajeron colores de los logos: navy `#25327a`, azul `#0165a4`, naranja `#f26c25`; acentos `--accent #2b58b8`, `--accent-hover #3568d0`.
- Logos copiados a `public/`: `eccsa.png` (splash), `hussmann.png` (topbar), `engrane.png` (empty-state).
- Icono del exe regenerado desde `pack.ico` del VB mediante `tauri icon`.

Motivo:
- Coherencia visual con la marca y el icono del VB.

Consecuencia:
- Cambios de marca van en `src/App.css` (variables CSS) y en `public/`.

## DEC-005 — Multi-zona con tablas por zona

Estado: ACTIVA

Decisión:
- Cada zona define su estación, nombres de tablas y conexión MAPICS/SQL (override). La zona activa se elige en config (`activeZone`).

Motivo:
- El VB separaba por zonas (p.ej. IMX Línea 7) con tablas `...IMX`.

Consecuencia:
- Consultas SQL construidas con `zone.tables.*`; no hay tablas hardcodeadas en `sql.rs`.