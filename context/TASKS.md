# Trabajo pendiente

## P0 - Crítico
- Ninguno.

## P1 - Importante
- Validar en la PC de producción: conexión SQL (tiberius/TCP) y MAPICS (ODBC/DSN) con la config real.
- Crear tabla `PackoutItemsImgIMX` en producción con `sql/crear_tabla_imagenes.sql` y cargar imágenes de prueba.
- Confirmar que WebView2 Runtime está disponible en la PC de producción.

## P2 - Normal
- Ajustar la estación de la zona por PC (ESTPACK01/ESTPACK02).
- Probar el flujo de auto-actualización: subir versión, hacer push a master, verificar release + actualización.
- Revisar/decidir el manejo de secretos si la config real debe viajar con el repo (NO: ver `SECURITY.md`).

## P3 - Futuro
- Migrar las imágenes existentes del VB (si aplica) a la tabla `PackoutItemsImgIMX`.
- Agregar más zonas si se requieren.
- Documentar el flujo de release en `SETUP.md` si cambia el proceso.