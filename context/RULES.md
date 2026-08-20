# Reglas del proyecto

## OBLIGATORIO

- Leer `STATE.md`, `INDEX.md` y `PROJECT.md` antes de tocar código.
- Respetar `DECISIONS.md` (sobre todo DEC-001: SQL por tiberius, no ODBC).
- **No ejecutar los tests de escritura contra la BD de producción** (inserta_y_recientes_ok, imagen_item_save_get_ok usan la config del `%APPDATA%`). Ver `ISSUES.md`.
- Para builds release usar `npm run tauri build` (NO `cargo build --release` a secas) — ver `ISSUES.md` ISSUE-002.
- Para actualizaciones: versionar en `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` y `package.json`; y firmar con las llaves del updater (ver `SECURITY.md`).
- Mantener el código VB.NET original (`HussmannPackout_imx/`) intacto: es la referencia de comportamiento.
- Las queries de MAPICS se mantienen como datos en la config (campo `query*` de cada zona), no hardcodeadas.

## RECOMENDADO

- Probar conexiones con el botón "Probar conexión" en SettingsPanel (prueba SQL + MAPICS).
- Documentar en `CHANGELOG.md` solo cambios relevantes de contexto (arquitectura/decisiones/estado), no cambios triviales.
- Mantener `context/` actualizado al cambiar arquitectura, tablas o decisiones.

## PROHIBIDO

- **NO subir secretos al repo público** (contraseñas, tokens, llaves). Repo `hector1516/packout` es **público**. Ver `SECURITY.md`.
- NO usar ODBC para SQL Server (DEC-001). ODBC es solo para MAPICS.
- NO regenerar iconos/logo manualmente: usar `npm run tauri icon` desde un PNG base.
- NO escribir SQL con interpolación sin sanitizar (usar `esc()` en sql.rs).
- NO agregar dependencias sin justificación; validar contra `Cargo.toml`/`package.json`.
- NO borrar el `context/` ni moverlo: es la memoria portable del proyecto.