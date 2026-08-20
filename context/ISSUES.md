# Problemas conocidos

## ISSUE-001 — No ejecutar tests de escritura contra producción

Estado: ABIERTO

Problema:
- Los tests en `sql.rs` cargan la config desde `%APPDATA%/com.packout.app/packout.config.json`, que en la PC de trabajo apunta a la BD real (10.96.16.114). Tests como `inserta_y_recientes_ok` e `imagen_item_save_get_ok` insertan/borran datos.

Causa conocida:
- Los tests usan la config activa del entorno, no una config de prueba aislada.

Workaround:
- Correr solo tests de lectura contra producción, o apuntar la config a una BD de pruebas antes de ejecutar los tests.

Solución pendiente:
- Aislar los tests con una config de prueba dedicada o un flag.

## ISSUE-002 — `cargo build --release` a secas produce exe que no abre (ERR_CONNECTION_REFUSED)

Estado: RESUELTO

Problema:
- Compilar el exe con `cargo build --release` (sin pasar por el build de Tauri) genera un binario que intenta usar el dev server `http://localhost:1420` y falla con `ERR_CONNECTION_REFUSED`.

Causa conocida:
- El frontend no se embebe correctamente si no se ejecuta el pipeline completo de Tauri (`beforeBuildCommand`/`frontendDist`).

Workaround:
- Usar siempre `npm run tauri build` para builds release.

Solución pendiente:
- Ninguna (resuelto por procedimiento).

## ISSUE-003 — LocalDB no sirve para probar SQL con tiberius

Estado: ABIERTO

Problema:
- LocalDB (SQL Server Express LocalDB) usa named pipes/shared memory; tiberius usa TCP por defecto, por lo que los tests contra LocalDB fallan (conexión rechazada).

Causa conocida:
- DEC-001 cambió la conexión a tiberius/TCP.

Workaround:
- Probar contra un SQL Server con TCP habilitado (p.ej. la instancia de producción, con cuidado de ISSUE-001).

Solución pendiente:
- Evaluar soporte de named pipes en tiberius para LocalDB, o mantener una instancia de prueba con TCP.

## ISSUE-004 — La PC de desarrollo no alcanza la red de producción

Estado: ABIERTO

Problema:
- `10.96.16.114` responde timeout (os error 10060) desde la PC de desarrollo; no hay acceso a la red donde vive el SQL Server.

Causa conocida:
- Segmentación de red (VPN/LAN).

Workaround:
- Validar contra SQL Server local con TCP, o directamente en la PC de producción.

Solución pendiente:
- Depende del entorno de red; no es un bug de la app.

## ISSUE-005 — SQL injection latente por `esc()`

Estado: ABIERTO

Problema:
- `esc()` en `sql.rs` solo reemplaza comillas simples `'`; las queries se construyen por interpolación de strings.

Causa conocida:
- Uso de concatenación de SQL para compatibilidad con el esquema existente.

Workaround:
- Validar/sanitizar entradas en frontend y comandos.

Solución pendiente:
- Migrar a queries parametrizadas de tiberius (bind params) cuando sea factible.