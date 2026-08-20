# Packout

## Nombre
Packout (antes "HussmannCabeceras" / app VB.NET).

## Propósito
Registrar y controlar el resultado de "packout" (armado de kits) en la línea IMX. El operador escanea/registra un número de serie, la app consulta MAPICS, valida el estado y guarda el resultado (APROBADO / rechazo) en SQL Server. Permite login de operador y administrador, historial, reimpresión de series y catálogo de imágenes por ítem.

## Problema que resuelve
Reemplaza una app de escritorio VB.NET (ClickOnce, obsoleta) por una app moderna Tauri, con instalador propio y actualizaciones automáticas vía GitHub Releases.

## Usuarios
- Operadores de línea (login con número de empleado).
- Administradores (login privilegiado).
- Personal de soporte / IT que mantiene el despliegue.

## Alcance
- Login operador + administrador contra tablas SQL.
- Consulta/inserción/borrado de kits en MAPICS (DB2 vía ODBC).
- Registro de resultados de packout en SQL Server.
- Historial y "recientes" por zona.
- Reimpresión de serie.
- Imágenes por ítem (almacenadas como base64 en SQL).
- Multi-zona (cada zona tiene su estación, tablas y conexiones).
- Auto-actualización desde GitHub Releases.

## Tecnologías principales
- Frontend: React + TypeScript + Vite (dev server en puerto 1420).
- Backend: Rust + Tauri 2.
- SQL Server: crate `tiberius` (protocolo TDS nativo, sin ODBC). Ver DEC-001.
- MAPICS (DB2): crate `odbc` con DSN. Ver DEC-001.
- Actualizaciones: `tauri-plugin-updater` + GitHub Releases. Ver DEC-002.
- Iconos: generados con `tauri icon`.

## Estado general
App funcional. En validación en la PC de producción. Ver `STATE.md`.