Imports System.Data.Odbc
Imports System.IO
Module mapics
    Public MapicsStringcon As String '= "DSN=datatest;UID=DATRINS;PWD=DATA18"
    Public MapicsServer As String
    Public MapicsQuery As String
    Public Sub Consulta(ByVal serie As String)
        Dim cmd As New OdbcCommand
        Try
            Form1.dtgMapics.DataSource = Nothing
            Form1.dtgMapics.Refresh()
            Form1.dtgMapics.Rows.Clear()
            'cmd.CommandText = "SELECT A.IMANUES, A.IMANULI, A.IMAORDE, A.IMANUSE, A.IMAKIT, A.IMACONF, A.IMADATE, B.EPCTIPO, B.EPCFECHA, B.EPCHORA, B.EPCNIMPR FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT WHERE A.IMANUES =770 AND A.IMANUSE  = '" & serie & "'"
            'cmd.CommandText = "SELECT A.IMANUES, A.IMANULI, A.IMAORDE, A.IMANUSE, A.IMAKIT, A.IMACONF, A.IMADATE, B.EPCTIPO, B.EPCFECHA, B.EPCHORA, B.EPCNIMPR FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT WHERE A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7') AND A.IMANUSE  = '" & serie & "'"
            'cmd.CommandText = MapicsQuery & " '" & serie & "'"
            cmd.CommandText = "SELECT A.IMANUES, A.IMANULI, A.IMAORDE, A.IMANUSE, A.IMAKIT, A.IMACONF, A.IMADATE, ifnull(B.EPCTIPO,'') as EPCTIPO, ifnull(B.EPCFECHA,0)as EPCFECHA, ifnull(B.EPCHORA,0) as EPCHORA, ifnull(B.EPCNIMPR,'') as EPCNIMPR, A.IMADATE as FechaRegEPC , A.IMAESKI FROM XACHGMEP.EPCIMAGE A left outer JOIN XACHGMEP.EPCBITA B ON A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT AND A.IMANUES = B.EPCESTAC AND A.IMANULI = B.EPCLINEA WHERE A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR = 7) AND A.IMAESKI <> 'Disabled' AND A.IMANUSE ='" & serie & "' ORDER BY A.IMADATE DESC"
            cmd.Connection = New OdbcConnection(MapicsStringcon)
            cmd.Connection.Open()
            Dim dr As OdbcDataReader = cmd.ExecuteReader
            Dim dt As New DataTable
            dt.Load(dr)
            Form1.dtgMapics.DataSource = dt
            cmd.Connection.Close()
            Sqll.Err("Rutina Mapics Consulta", "Rutina Mapics ejecutada: " & cmd.CommandText.ToString)
        Catch ex As Exception
            Sqll.Err("Error Mapics", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub insert(ByVal serie As String)
        Dim cmd As New OdbcCommand
        Try
            cmd.CommandText = MapicsQuery & serie & "' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')" '"INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE', 'ESTPACK02', CURRENT DATE, CURRENT TIME, 'ECCSA' FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '" & serie & "' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')"
            'cmd.CommandText = "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE, 'ESTPACK01', CURRENT DATE, CURRENT TIME, 'ECCSA' FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '" & serie & "' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')"
            cmd.Connection = New OdbcConnection(MapicsStringcon)
            cmd.Connection.Open()
            cmd.ExecuteNonQuery()
            cmd.Connection.Close()
            cmd.Connection.Dispose()
            Sqll.Err("Rutina Mapics Insertar", "Rutina Mapics ejecutada: " & cmd.CommandText.ToString)
        Catch ex As Exception
            Sqll.Err("Error Mapics", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub borrar(ByVal serie As String)
        Dim cmd As New OdbcCommand
        Try
            cmd.CommandText = "DELETE FROM XACHGMEP.FESRLKIT WHERE KPSRLN = '" & serie & "'"
            cmd.Connection = New OdbcConnection(MapicsStringcon)
            cmd.Connection.Open()
            cmd.ExecuteNonQuery()
            cmd.Connection.Close()
            cmd.Connection.Dispose()
            Sqll.Err("Rutina Mapics Borrar", "Rutina Mapics ejecutada: " & cmd.CommandText.ToString)
        Catch ex As Exception
            Sqll.Err("Error Mapics", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
End Module
