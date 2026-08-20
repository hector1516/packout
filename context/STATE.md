# Estado actual del proyecto

## COMPLETADO
- Port de la app VB.NET a Tauri 2 + React (login, historial, recientes, imágenes, reimpresión, errores).
- SQL Server por tiberius (DEC-001); ODBC solo MAPICS.
- Updater GitHub Releases con firmado (DEC-002); comandos `check_update`/`install_update`.
- Config persistente JSON con export/import (DEC-003).
- Branding de marca + icono del exe (DEC-004).
- Build release firmado OK (exe 14 MB, NSIS 3.66 MB, MSI 5.5 MB) con `npm run tauri build`.
- Repo `github.com/hector1516/packout` con commit inicial `65dd5a1`, rama `master`.
- Secrets de GitHub configurados: `TAURI_SIGNING_PRIVATE_KEY` y `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (ver `SECURITY.md`).
- Workflow de release: `.github/workflows/release.yml` (en master).

## EN DESARROLLO
- Validación del exe en la PC de producción (conexión SQL vía tiberius, MAPICS vía ODBC, WebView2).

## PENDIENTE
- Validar la app en la PC de producción (SQL vía tiberius, MAPICS vía ODBC, WebView2).
- Crear la tabla de imágenes `PackoutItemsImgIMX` en producción y cargar imágenes de prueba.
- Ajustar estación por PC (ESTPACK01/ESTPACK02).

Detalle y prioridades: ver `TASKS.md`.

## BLOQUEADO
- Ninguno.

## PRÓXIMO PASO
- En la PC de producción: copiar exe/instalador + config, verificar "Probar conexión" (SQL + MAPICS), crear tabla de imágenes y probar el flujo completo.