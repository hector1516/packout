# Setup / Cómo levantar el proyecto

## Requisitos

- Node.js (con npm) y Rust toolchain (cargo).
- Tauri CLI (vía npm, en `package.json`).
- En la PC de destino: WebView2 Runtime (Windows 10 actualizado normalmente lo incluye) — el instalador NSIS/MSI lo instala.
- Para conexión SQL: SQL Server accesible por TCP (puerto 1433 por defecto).
- Para conexión MAPICS: driver ODBC y DSN configurado (ver `INTEGRATIONS.md`).

## Variables de entorno (NO secretos reales)

- `TAURI_SIGNING_PRIVATE_KEY` — contenido de la llave privada del updater (ver `SECURITY.md`).
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — password de la llave.
- `COMPUTERNAME` — usada para el log local (el sistema la provee).

## Comandos

- Instalar dependencias: `npm install`
- Dev (con hot reload y Vite): `npm run tauri dev`
- Build de frontend: `npm run build`
- Build release completo (frontend + bundle + firma): `npm run tauri build`
  - Requiere `TAURI_SIGNING_PRIVATE_KEY` y `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` antes de ejecutar.
  - NO usar `cargo build --release` a secas (ISSUE-002).
- Iconos: `npm run tauri icon <png-base>`
- Tests: `cargo test` en `src-tauri/` (ojo: usan la config del `%APPDATA%` — ISSUE-001).

## Build release / actualizaciones

- La versión se define en 3 lugares: `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `package.json`.
- Para publicar una actualización: subir la versión en los 3, hacer build con firma, commit + push a `master` (dispara `.github/workflows/release.yml`).

## Despliegue en PC de producción

1. Copiar la config real a `%APPDATA%/com.packout.app/packout.config.json` (o importarla desde la UI).
2. Instalar el exe/instalador (NSIS o MSI) o copiar el exe suelto (requiere WebView2).
3. Crear la tabla de imágenes si no existe: `sql/crear_tabla_imagenes.sql`.
4. Verificar con "Probar conexión" en Configuración (SQL + MAPICS).
5. Revisar `%APPDATA%/com.packout.app/packout.log` ante fallos.