# Seguridad

## Principio general
El repositorio `github.com/hector1516/packout` es **PÚBLICO**. **No** debe contener secretos. Este archivo NO guarda valores reales; documenta dónde viven y cómo se manejan.

## Credenciales de la app
- Se guardan en `packout.config.json` (`%APPDATA%/com.packout.app/`) y se cargan desde `config.rs`.
- La config de producción NO debe subirse al repo.
- El archivo de trabajo en el entorno de desarrollo apunta a la BD real (SQL 10.96.16.114 / MAPICS DSN datatest).

## Llaves del updater
- Ubicación: `C:\Users\hecto\.tauri\packout.key` (privada) y `packout.key.pub` (pública).
- La llave privada NO se sube al repo; se expone al build como variables de entorno:
  - `TAURI_SIGNING_PRIVATE_KEY` (contenido de `packout.key`)
  - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- En GitHub se almacenan como secrets: `TAURI_SIGNING_PRIVATE_KEY` y `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.
- La clave pública (segura) está en `src-tauri/tauri.conf.json` → `plugins.updater.pubkey`.
- Nota de despliegue: `TAURI_SIGNING_PRIVATE_KEY_PATH` NO funciona en este pipeline; se usa el contenido directo.

## Autenticación en la app
- Login de operador y administrador validado contra tablas SQL (`PackoutUsrIMX` / `PackoutAdminIMX`).
- Roles: operador y administrador (el admin puede reimprimir, entre otros).

## Riesgos conocidos
- `esc()` en `sql.rs` solo elimina comillas simples; riesgo de SQL injection latente (ISSUE-005). Preferir queries parametrizadas.
- La config con credenciales reales viaja en cada PC en `%APPDATA%`; no cifrada.
- Repo público: cualquier secret que se agregue por error queda expuesto (los secrets de GitHub no entran en el repo).

## Reglas de seguridad
- PROHIBIDO: commitear tokens, contraseñas, llaves o configs con credenciales reales.
- PROHIBIDO: publicar `packout.key` o `packout.config.json` con secretos.
- RECOMENDADO: rotar credenciales si alguna vez se filtra.