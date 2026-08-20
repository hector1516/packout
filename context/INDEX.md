# Packout — Memoria portable del proyecto

Port de la app VB.NET "HussmannCabeceras" a Tauri 2 + React. Registro de resultados de "packout" en línea IMX, consulta de series contra MAPICS (DB2) e inserción de resultados en SQL Server.

## Mapa de archivos

```
PROJECT.md       → Qué es el proyecto, usuarios, alcance, tecnologías.
ARCHITECTURE.md  → Componentes, flujo principal, tablas y datos.
RULES.md         → Reglas obligatorias/recomendadas/prohibidas.
DECISIONS.md     → Decisiones importantes (DEC-XXX).
STATE.md         → Estado actual del proyecto.
TASKS.md         → Trabajo pendiente priorizado (P0–P3).
ISSUES.md        → Problemas conocidos y workarounds.
SETUP.md         → Cómo instalar, compilar y ejecutar.
SECURITY.md      → Seguridad, credenciales, riesgos (sin secretos).
INTEGRATIONS.md  → SQL Server, MAPICS y GitHub Releases.
CHANGELOG.md     → Cambios recientes de contexto.
```

## LECTURA RECOMENDADA

Siempre:
- `INDEX.md`
- `PROJECT.md`
- `RULES.md`
- `STATE.md`

Solo cuando sea necesario:
- `ARCHITECTURE.md`
- `DECISIONS.md`
- `INTEGRATIONS.md`
- `SETUP.md`
- `SECURITY.md`
- `ISSUES.md`
- `TASKS.md`

## Notas iniciales

- Repo: `https://github.com/hector1516/packout.git` — rama `master`, repositorio **público**. Solo 1 commit inicial (`65dd5a1`).
- El código VB.NET original vive en `HussmannPackout_imx/` y sirve como referencia de comportamiento (NO se modifica).
- No ejecutar los tests de escritura contra producción: usan la config del `%APPDATA%` que apunta a la BD real. Ver `ISSUES.md`.