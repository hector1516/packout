# Base de datos del proyecto Visual Basic original (`HussmannPackout_imx`)

Extraído del código fuente VB.NET. Fuente principal: `HussmannPackout_imx/Sqll.vb`,
`HussmannPackout_imx/mapics.vb`, `HussmannPackout_imx/Form1.vb`, `HussmannPackout_imx/Pruebas.vb`.

> ⚠️ Las contraseñas reales NO se documentan aquí (repo público).
> La de producción vive solo en `%APPDATA%\com.packout.app\packout.config.json` de la PC de producción.

> ⚠️ **OJO — `LINEPR`: 7 vs 8.** Los queries copiados del VB usan `LINEPR = 7`
> (el VB original solo operaba la línea 7). **Esta app usa `LINEPR = 8`**,
> confirmado por el área de MAPICS. Los valores de abajo son los del VB;
> los de la app están en `src-tauri/src/config.rs` (`DEFAULT_CONFIG`).

> ⚠️ **Los queries son editables por zona en Ajustes → MAPICS.** `DEFAULT_CONFIG`
> solo aplica a instalaciones nuevas: si el `%APPDATA%\...\packout.config.json`
> ya existe, conserva las queries anteriores hasta que se editen y guarden
> desde la app.

## 1. SQL Server (`Sqll.vb:8-11`)

| Dato            | Valor (VB original)              |
|-----------------|----------------------------------|
| `Data Source`   | `10.96.16.114`                   |
| `Initial Catalog` | `hussmann_insight`             |
| `User ID`       | `HInsightUser`                   |
| `Password`      | *(ver config de producción)*     |
| `MultipleActiveResultSets` | `False`               |
| Variable ping   | `ServerSQL = "10.96.16.114"`     |

Cadena original (`Sqll.vb:8`):
`Data Source=10.96.16.114;Initial Catalog=hussmann_insight;MultipleActiveResultSets=False; User ID=HInsightUser;Password=***`

Chequeo de línea: `checkSvrOnline()` hace **ping** (2s timeout) al servidor y pone `lblSQL = "SQL OK" / "SQL Not OK"`.

### 1.1 Tablas SQL usadas

| Tabla                  | Uso en VB |
|------------------------|-----------|
| `PackoutResultadosIMX` | `insrt()` INSERT de cada kit aprobado (FechaHora, Pedido, Serie, Resultado, Operador, OperadorAdmin, Comentario) · `ConsultaResultados()` SELECT TOP 10 ORDER BY FechaHora DESC |
| `PackoutErrIMX`        | `Err()` INSERT de errores (FechaHora, Titulo, [Desc]) |
| `PackoutUsrIMX`        | `ConsultaUsr()` SELECT * WHERE No = '...' (colaboradores) |
| `PackoutAdminIMX`      | `ConsultaUsrAdmin()` y `ConsultaUsrAdminLogin()` SELECT * WHERE No = '...' (admins/login) |
| `PackoutResViewIMX`    | `ConsultaUltimosSeriesPackout()` SELECT TOP 20 WHERE Resultado='APROBADO' (combo de reimpresión) |

### 1.2 Queries SQL exactos del VB

```sql
-- Err() · Sqll.vb:26
INSERT INTO PackoutErrIMX (FechaHora, Titulo, [Desc]) VALUES('...', '...', '...')

-- ConsultaResultados() · Sqll.vb:54
SELECT TOP 10 * FROM PackoutResultadosIMX ORDER BY FechaHora DESC

-- ConsultaUsr() · Sqll.vb:72
SELECT * FROM PackoutUsrIMX WHERE No = '...'

-- ConsultaUsrAdmin() / Login · Sqll.vb:89,182
SELECT * FROM PackoutAdminIMX WHERE No = '...'

-- insrt() · Sqll.vb:111
INSERT INTO PackoutResultadosIMX (FechaHora, Pedido, Serie, Resultado, Operador, OperadorAdmin, Comentario) VALUES('...', ...)

-- ConsultaUltimosSeriesPackout() · Sqll.vb:140
SELECT TOP 20 * FROM PackoutResViewIMX WHERE Resultado = 'APROBADO' ORDER BY FechaHora DESC
```

Formato `FechaHora`: `yyyy/MM/dd HH:mm:ss` (ej: `2026/09/28 14:35:10`).

## 2. MAPICS / AS400 por ODBC (`mapics.vb`, `Form1.vb:307-316`)

| Entorno   | DSN        | UID       | Servidor            |
|-----------|------------|-----------|---------------------|
| Producción| `datatest` | `DATRINS` | `prod.hussmann.com` |
| Pruebas   | `test`     | `DATRINS` | `test.hussmann.com` |

Se elige con el checkbox "Servidor Mapics Pruebas" (`Form1.vb:22-26`, `CheckBoxMapicsServer_CheckedChanged`).
Conexión: `System.Data.Odbc` (`OdbcCommand` + `OdbcConnection` + `OdbcDataReader`).

### 2.1 Tablas MAPICS usadas

| Tabla (esquema `XACHGMEP`) | Uso |
|-----------------------------|-----|
| `EPCIMAGE`  | Maestro serie → kits (alias `A`: IMANUES, IMANULI, IMAORDE, IMANUSE, IMAKIT, IMACONF, IMADATE, IMAESKI) |
| `EPCBITA`   | Historial de impresión (alias `B`: EPCTIPO, EPCFECHA, EPCHORA, EPCNIMPR, EPCSERIE, EPCKIT, EPCESTAC, EPCLINEA) |
| `EPC002PF`  | Filtro de estaciones de línea 7 (`SELECT NUESTA ... WHERE LINEPR = 7`) |
| `FESRLKIT`  | Destino del registro PACKOUT COMPLETE (clave `KPSRLN` = serie) |
| `BAN100PF`  | Maestro de serie/orden (INSLIN, INSPED, INSNEQ, INSITE, INSMOO, INSSER, INSSCU) |

### 2.2 Queries MAPICS exactos del VB

```sql
-- Consulta() · mapics.vb:16
SELECT A.IMANUES, A.IMANULI, A.IMAORDE, A.IMANUSE, A.IMAKIT, A.IMACONF, A.IMADATE,
  ifnull(B.EPCTIPO,'') as EPCTIPO, ifnull(B.EPCFECHA,0) as EPCFECHA,
  ifnull(B.EPCHORA,0) as EPCHORA, ifnull(B.EPCNIMPR,'') as EPCNIMPR,
  A.IMADATE as FechaRegEPC, A.IMAESKI
FROM XACHGMEP.EPCIMAGE A
LEFT OUTER JOIN XACHGMEP.EPCBITA B ON A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT
  AND A.IMANUES = B.EPCESTAC AND A.IMANULI = B.EPCLINEA
WHERE A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR = 7)
  AND A.IMAESKI <> 'Disabled' AND A.IMANUSE ='<serie>' ORDER BY A.IMADATE DESC

-- insert() · vía MapicsQuery (Form1.vb:432-441) + serie
INSERT INTO XACHGMEP.FESRLKIT
SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE, C.INSMOO, C.INSSER, C.INSSCU,
  'PACKOUT COMPLETE', '<ESTACION>', CURRENT DATE, CURRENT TIME, 'ECCSA'
FROM XACHGMEP.EPCIMAGE A
INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT
INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER
WHERE A.IMANUSE = '<serie>' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')

-- borrar() · mapics.vb:49
DELETE FROM XACHGMEP.FESRLKIT WHERE KPSRLN = '<serie>'
```

## 3. Estaciones por nombre de PC (`Form1.vb:401-447`)

| PC (`ComputerName`) | Estación   |
|---------------------|------------|
| `MRYD-37W0H63`      | `ESTPACK01`|
| `MRYD-1NZPMN2`      | `ESTPACK02`|
| `MRYL-DMQQ2D3`      | `ESTPACK01`|
| cualquier otra      | `ESTPACK01` (default) |

La estación se incrusta en el `INSERT` de MAPICS (`'PACKOUT COMPLETE', '<ESTACION>', ...`).

## 4. Equivalencia en la app Tauri actual

| VB original | Tauri actual |
|-------------|--------------|
| `Sqll.sqlstringcon` / `ServerSQL` | `config.sql` (`server`, `database`, `user`, `password`) en `%APPDATA%\com.packout.app\packout.config.json` |
| `checkSvrOnline()` (ping) | `test_zone` + `useConnectivity` (health-check 20s) |
| `MapicsStringcon` (DSN) | `zone.mapics` (`server`, `dsn`, `user`, `password`, queries editables) |
| Checkbox servidor pruebas | Campo `server`/`dsn` por zona en Ajustes |
| `SelImpresora()` por PC | Campo `estacion` por zona en Ajustes |
| Queries fijos en código | Queries editables en Ajustes (`queryKit`, `queryInsert`, `queryDelete`, `queryBuffer`) |
| `SELECT 1 FROM SYSIBM.SYSDUMMY1` (implícito en el chequeo) | Botón **🔌 Probar conexión MAPICS** en Ajustes → MAPICS (comando `mapics_test`) |

## 5. Configuración actual de la app (línea 8)

Queries en `src-tauri/src/config.rs` (`DEFAULT_CONFIG`), con `LINEPR = 8`:

| Query | Placeholders | `LINEPR` |
|-------|--------------|----------|
| `queryKit`    | `{SERIE}` | `= 8` |
| `queryInsert` | `{SERIE}`, `{ESTACION}` | `='8'` |
| `queryDelete` | `{SERIE}` | — |
| `queryBuffer` | `{SERIE}`, `{LIMIT}` | `= 8` |
| `mapics_test` | — | query fija `SELECT 1 FROM SYSIBM.SYSDUMMY1` |

`{SERIE}` / `{ESTACION}` / `{LIMIT}` se sustituyen en `config::render()` antes de ejecutar.

