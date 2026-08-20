# Integraciones / Servicios externos

No se documentan secretos aquí (ver `SECURITY.md`).

## SQL Server (hussmann_insight)
- Acceso: `sql.rs` con tiberius (TDS nativo, TCP 1433 por defecto), `EncryptionLevel::Required` + trust cert.
- Servidor (producción, según config real): `10.96.16.114`, base `hussmann_insight`.
- Credenciales: usuario `HInsightUser` (password en config local, NO en este repo).
- Tablas por zona (zona `imx`, prefijo `IMX`):
  - `PackoutResultadosIMX` — resultados de packout (FechaHora, Pedido, Serie, Resultado, Operador, OperadorAdmin, Comentario).
  - `PackoutErrIMX` — errores (FechaHora, Titulo, Desc).
  - `PackoutUsrIMX` — operadores (login).
  - `PackoutAdminIMX` — administradores (login).
  - `PackoutResViewIMX` — vista de "recientes" (Resultado = APROBADO).
  - `PackoutItemsImgIMX` — imágenes por ítem (Item, Imagen base64, FechaHora). **Puede no existir en producción**: crear con `sql/crear_tabla_imagenes.sql`.
- Logs locales también en `packout.log`.

## MAPICS (DB2, sistema legado)
- Acceso: `mapics.rs` con ODBC (`DSN=...;UID=...;PWD=...`).
- DSN de producción: `datatest`; usuario `DATRINS` (password en config local).
- Servidor: `prod.hussmann.com`.
- Tablas usadas (esquema `XACHGMEP.*`): `EPCIMAGE`, `EPCBITA`, `BAN100PF`, `EPC002PF`, `FESRLKIT`.
- Operaciones (queries en config de cada zona, campo `mapics`):
  - `queryKit` — consulta de kit por serie (INSERT/UPDATE se definen en config).
  - `queryInsert` — inserta kit en `FESRLKIT` al confirmar packout.
  - `queryDelete` — borra de `FESRLKIT` al reimprimir.
- `test_connection` ejecuta `SELECT 1 FROM SYSIBM.SYSDUMMY1`.

## GitHub Releases (actualizaciones)
- Repo: `https://github.com/hector1516/packout` (público, rama `master`).
- Endpoint del updater: `https://github.com/hector1516/packout/releases/latest/download/latest.json`.
- Flujo: push a `master` dispara `.github/workflows/release.yml` → build + publica release con `latest.json` y firmas `.sig`.
- Requiere secrets: `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.
- Plugin: `tauri-plugin-updater` (`installMode: passive`); comandos `check_update`/`install_update` en `updater.rs`.
- Nota: el check automático falla en silencio; el botón "Buscar actualizaciones" muestra errores.
- Para una actualización real: subir versión en `tauri.conf.json` + `Cargo.toml` + `package.json`, build con firma y push.

## Otros plugins
- `tauri-plugin-opener`, `tauri-plugin-dialog` (usado para export/import de config).