# Changelog de contexto

## 2026-08-19
- [ARCHITECTURE] Port VB.NET → Tauri 2 + React completado (login, historial, imágenes, reimpresión, errores).
- [DECISION] DEC-001: SQL Server por tiberius (nativo), ODBC solo MAPICS.
- [DECISION] DEC-002: auto-actualización vía GitHub Releases + firmado.
- [DECISION] DEC-003: config persistente JSON por zona con export/import.
- [DECISION] DEC-004: branding con colores/logos Hussmann/ECCSA e icono regenerado.
- [DECISION] DEC-005: multi-zona con tablas por zona.
- [INTEGRATION] Updater GitHub configurado (endpoint `latest.json`), secrets creados, workflow `release.yml` en master.
- [STATE] Build release firmado OK (`npm run tauri build`); exe 14 MB, NSIS 3.66 MB, MSI 5.5 MB.
- [ISSUE] ISSUE-002 resuelto (build por `npm run tauri build`; `cargo build --release` a secas da ERR_CONNECTION_REFUSED).
- [SECURITY] Regla: no subir secretos al repo público.

## 2026-08-19 (posterior, mismo día)
- [ARCHITECTURE] Refactor de `sql.rs`: migración de ODBC a tiberius completada y compilada (`cargo check` OK).
- [STATE] Tests de lectura fallan por red (timeout a 10.96.16.114, ISSUE-004); tests de escritura no deben correrse contra producción (ISSUE-001).
- [INTEGRATION] Se validó que LocalDB no es compatible con tiberius (ISSUE-003).
- [SETUP] Se documentó que `TAURI_SIGNING_PRIVATE_KEY` debe usarse por contenido, no por PATH.
- [CONTRADICCIÓN] El `CHANGELOG.md` de la raíz (líneas 17 y 21) aún describe SQL con ODBC y tests sobre LocalDB. Tras DEC-001 eso quedó obsoleto: SQL usa tiberius (TCP) y LocalDB no es compatible. La información más reciente está en este `context/`.