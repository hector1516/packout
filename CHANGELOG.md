# Changelog

Todas las notas de los cambios relevantes de la aplicación Packout.

El formato se basa en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/), y este proyecto respeta [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-08-10

Primera versión funcional de Packout sobre Tauri 2 + React. Port completo de la aplicación legada VB.NET `HussmannCabeceras`.

### Añadido

#### Núcleo (Backend Rust)
- Configuración por zona guardada en `%APPDATA%\com.packout.app\packout.config.json` con valores por defecto y auto-creación en el primer arranque.
- Soporte multi-zona: cada zona define nombre, estación, tablas SQL, conexión MAPICS (DSN) y sus queries.
- Nombres de zona, tablas y DSN 100% variables por config; "IMX" solo aparece como valor por defecto inicial.
- Conexión ODBC a SQL Server y MAPICS con driver configurable (`SqlDb.driver`).
- Prevención de dependencia absurda: sin credenciales hardcodeadas (default `CAMBIAME`), sin backdoor `Qwe123456`.
- Comandos Tauri: `get_config`, `save_config`, `set_active_zone`, `test_zone`, `mapics_query_kit`, `mapics_insert_kit`, `mapics_delete_kit`, `sql_historial`, `sql_recientes`, `sql_insert_resultado`, `sql_insert_error`, `sql_login`, `sql_check_operator`, `reimprimir`, `sql_item_image`, `sql_save_item_image`, `sql_delete_item_image`, `sql_list_item_images`.
- Log local `packout.log` en `%APPDATA%` sin credenciales + registro de errores en BD.
- 6 tests sobre base SQL LocalDB (conexión, historial, admin, operador, insert+recientes, imágenes).

#### Flujo de escaneo (Frontend React)
- Máquina de estados `idle → kit → approved → done` replicando la lógica de 3 banderas del VB (`vali1/vali2/vali3`).
- Validación de seriales `MY*/my*`, limpieza de `()`, espacios y mayúsculas.
- Construcción del kit idéntica al VB: filas `Disabled` se omiten, `IMACONF=Y` + `EPCNIMPR` con `A` incrementan conteo, claves por estación `90/790/810/910` (`IMANUSE+EPCNIMPR`) vs `XX1`, y `IMACONF!=Y` → `XX3`.
- Escaneo de items marcándolos (gris y `✓ escaneado`) en vez de eliminarlos de la grilla; al completarse dispara `mapics_insert_kit` y pide gafete de operador.
- Validaciones: serial inválido, código no perteneciente al kit, item ya escaneado, operador no registrado.

#### Funcionalidades
- Pantalla principal con login de admin, registro pendiente, manual, reimpresión (delete → insert → resultado `REIMPRESO` → log) e imágenes.
- Fotos de items almacenadas en BD (base64, tabla `PackoutItemsImgIMX`) en lugar de filesystem; imagen genérica SVG para items sin foto.
- Historial y series recientes desde la BD.
- Splash de carga y banner de errores.
- Tema oscuro: fondo negro `#000`, paneles `#0b101a`, acentos azul/branding.

#### Branding
- Icono del ejecutable regenerado desde el `pack.ico` del proyecto VB original.
- Colores de marca extraídos de los logos (azul Hussmann `#0165A4`, navy ECCSA `#25327A`, naranja `#F26C25`) aplicados sutilmente al tema.
- Logos visibles: ECCSA en splash, Hussmann en topbar, engrane en empty-state.

### Configuración / Import-Export
- Exportar configuración a archivo `.config` y reimportarla (diálogos nativos de guardar/abrir).

### Actualizaciones (Updater)
- Integración `tauri-plugin-updater` firmada (minisign) con GitHub Releases como endpoint.
- Búsqueda automática de actualización al arranque (silenciosa si no hay release), botón "Buscar actualizaciones" y banner de "Nueva versión disponible" con descarga/instalación.
- Artifacts de actualización generados en el build (`.sig`) para MSI y NSIS.
- Workflow GitHub Actions `.github/workflows/release.yml` que compila, firma y publica release en cada push a `master`.

### Corregido
- Debug exe falla con `ERR_CONNECTION_REFUSED` (requiere Vite en puerto 1420); el release exe funciona independiente.
- Compilación de la dependencia `tauri-plugin-updater` (cfg de target con quoting correcto).
- Error del updater "Could not fetch a valid release JSON" al no existir release publicado: el check automático ahora falla en silencio.

### Notas de despliegue
- Producción requiere el instalador (MSI/NSIS), no el exe suelto, para soportar actualizaciones.
- En cada máquina: WebView2 Runtime + ODBC Driver 17 (o el configurado) + DSN MAPICS `datatest`.