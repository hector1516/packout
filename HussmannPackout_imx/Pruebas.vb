Imports System.Data.Odbc
Public Class Pruebas
    Private Sub Pruebas_Load(sender As Object, e As EventArgs) Handles MyBase.Load
    End Sub
    Public Sub insertESTACK01(ByVal serie As String)
        Dim cmd As New OdbcCommand
        Try
            'cmd.CommandText = "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE', '" & Form1.localizacion & "', CURRENT DATE, CURRENT TIME, 'ECCSA'FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '" & serie & "' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')"
            cmd.CommandText = "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE', 'ESTPACK01', CURRENT DATE, CURRENT TIME, 'ECCSA' FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '" & serie & "' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')"
            cmd.Connection = New OdbcConnection(mapics.MapicsStringcon)
            cmd.Connection.Open()
            cmd.ExecuteNonQuery()
            cmd.Connection.Close()
            cmd.Connection.Dispose()
            txtLog.Text = DateTime.Now.ToString & vbNewLine & "Rutina Mapics ejecutada con exito: " & cmd.CommandText.ToString & vbNewLine & mapics.MapicsStringcon
        Catch ex As Exception
            txtLog.Text = DateTime.Now.ToString & vbNewLine & "Error: " & ex.ToString
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub insertESTACK02(ByVal serie As String)
        Dim cmd As New OdbcCommand
        Try
            'cmd.CommandText = "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE', '" & Form1.localizacion & "', CURRENT DATE, CURRENT TIME, 'ECCSA'FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '" & serie & "' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')"
            cmd.CommandText = "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE', 'ESTPACK02', CURRENT DATE, CURRENT TIME, 'ECCSA' FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '" & serie & "' AND A.IMANUES IN (SELECT NUESTA FROM XACHGMEP.EPC002PF WHERE LINEPR='7')"
            cmd.Connection = New OdbcConnection(mapics.MapicsStringcon)
            cmd.Connection.Open()
            cmd.ExecuteNonQuery()
            cmd.Connection.Close()
            cmd.Connection.Dispose()
            txtLog.Text = DateTime.Now.ToString & vbNewLine & "Rutina Mapics ejecutada con exito: " & cmd.CommandText.ToString & vbNewLine & mapics.MapicsStringcon
        Catch ex As Exception
            txtLog.Text = DateTime.Now.ToString & vbNewLine & "Error: " & ex.ToString
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub borrar(ByVal serie As String)
        Dim cmd As New OdbcCommand
        Try
            cmd.CommandText = "DELETE FROM XACHGMEP.FESRLKIT WHERE KPSRLN = '" & serie & "'"
            cmd.Connection = New OdbcConnection(mapics.MapicsStringcon)
            cmd.Connection.Open()
            cmd.ExecuteNonQuery()
            cmd.Connection.Close()
            cmd.Connection.Dispose()
            txtLog.Text = DateTime.Now.ToString & vbNewLine & "Rutina Mapics ejecutada con exito: " & cmd.CommandText.ToString & vbNewLine & mapics.MapicsStringcon
        Catch ex As Exception
            txtLog.Text = DateTime.Now.ToString & vbNewLine & "Err: " & ex.ToString
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Private Sub btnInsert_Click(sender As Object, e As EventArgs) Handles btnInsert.Click
        btnInsert.Enabled = False
        If txtSerieInsert.Text.Length > 6 Then
            insertESTACK01(txtSerieInsert.Text.ToUpper)
        End If
        btnInsert.Enabled = True
    End Sub
    Private Sub Button1_Click(sender As Object, e As EventArgs) Handles Button1.Click
        Button1.Enabled = False
        If txtSerieInsert.Text.Length > 6 Then
            insertESTACK02(txtSerieInsert.Text.ToUpper)
        End If
        Button1.Enabled = True
    End Sub
    Private Sub btnDel_Click(sender As Object, e As EventArgs) Handles btnDel.Click
        btnDel.Enabled = False
        If txtSerieInsert.Text.Length > 6 Then
            borrar(txtSerieInsert.Text.ToUpper)
        End If
        btnDel.Enabled = True
    End Sub
    Private Sub txtSerieInsert_TextChanged(sender As Object, e As EventArgs) Handles txtSerieInsert.TextChanged
        txtSerieInsert.Text = txtSerieInsert.Text.Replace("(", "")
    End Sub

End Class