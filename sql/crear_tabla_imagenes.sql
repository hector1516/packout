-- Packout (Tauri) - Script de migración para producción
-- Ejecutar contra: 10.96.16.114 / hussmann_insight

-- 1) Tabla de imágenes de items (la única tabla que falta vs la app VB.NET)
IF OBJECT_ID('PackoutItemsImgIMX', 'U') IS NULL
CREATE TABLE PackoutItemsImgIMX (
    Item      VARCHAR(50)  NOT NULL,
    Imagen    NVARCHAR(MAX) NULL,   -- imagen en base64
    FechaHora VARCHAR(30)  NULL,
    CONSTRAINT PK_PackoutItemsImgIMX PRIMARY KEY (Item)
);
GO

-- 2) Verificación de tablas existentes (deben estar: son las del VB)
SELECT name FROM sys.tables
WHERE name IN (
    'PackoutResultadosIMX',
    'PackoutErrIMX',
    'PackoutUsrIMX',
    'PackoutAdminIMX',
    'PackoutResViewIMX',
    'PackoutItemsImgIMX'
) ORDER BY name;