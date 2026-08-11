Imports System.Data.SqlClient
Imports System.IO
Imports System.Net.NetworkInformation

Module Sqll
    'Dim lector As New StreamReader("C:\ECCSA\HUSS\PackOut\Recursos\sql.ini")
    'Public sqlstringcon As String = lector.ReadLine
    Public sqlstringcon As String = "Data Source=10.96.16.114;Initial Catalog=hussmann_insight;MultipleActiveResultSets=False; User ID=HInsightUser;Password=PatcONS3St3Nts1d"
    Dim con As New SqlConnection(sqlstringcon)
    Dim cmd As New SqlCommand
    Public ServerSQL As String = "10.96.16.114"
    Public Sub Err(ByVal titulo As String, ByVal desc As String)
        Dim dia As String = DateTime.Now.ToString("yyyy/MM/dd")
        Dim hora As String = DateTime.Now.ToString("HH:mm:ss")
        Dim fechahora As String = dia & " " & hora
        Dim fecha As String = DateTime.Now.ToString("dd/MM/yyyy HH:mm:ss:ff")
        Form1.lblError.Text = titulo
        Beep()
        Try
            If checkSvrOnline(ServerSQL) = True Then
                If con.State = 1 Then
                    con.Close()
                End If
                con.Open()
                cmd.Connection = con
                cmd.CommandText = "INSERT INTO PackoutErrIMX (FechaHora, Titulo, [Desc]) VALUES('" _
                & fechahora & "', '" _
                & Form1.localizacion & " - " & titulo & "', '" _
                & desc.Replace("'", "") & "')"
                cmd.ExecuteNonQuery()
            End If
            Try
                Form1.dtg_error.Rows.Insert(0, fecha, titulo, desc)
                If Form1.dtg_error.Rows.Count = 999 Then
                    Form1.dtg_error.Rows.Clear()
                    Form1.dtg_error.Rows.Insert(0, fecha, "logg", "Se llego al maximo numero de registros (999)")
                End If
            Catch ex As Exception
            End Try
        Catch ex As Exception
            'MsgBox("Error en mensaje de error: " & ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        Finally
            con.Close()
        End Try
    End Sub
    Public Sub ConsultaResultados()
        Dim dt As New DataTable()
        Try
            If checkSvrOnline(ServerSQL) = True Then
                Form1.dtgHistorico.DataSource = Nothing
                Form1.dtgHistorico.Refresh()
                Form1.dtgHistorico.Rows.Clear()
                Dim sql_command As String = "SELECT TOP 10 * FROM " & "PackoutResultadosIMX ORDER BY FechaHora DESC"
                Dim adapter As New SqlDataAdapter(sql_command, con)
                adapter.Fill(dt)
                Form1.dtgHistorico.DataSource = dt
                Form1.dtgHistorico.Columns(7).AutoSizeMode = DataGridViewAutoSizeColumnMode.Fill
            End If
        Catch ex As Exception
            Sqll.Err("Error consulta de Resultados", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub ConsultaUsr(ByVal user As String)
        Dim dt As New DataTable()
        Try
            If checkSvrOnline(ServerSQL) = True Then
                Form1.dtgUsers.DataSource = Nothing
                Form1.dtgUsers.Refresh()
                Form1.dtgUsers.Rows.Clear()
                Dim sql_command As String = "SELECT * FROM PackoutUsrIMX WHERE No = '" & user & "'"
                Dim adapter As New SqlDataAdapter(sql_command, con)
                adapter.Fill(dt)
                Form1.dtgUsers.DataSource = dt
            End If
        Catch ex As Exception
            Sqll.Err("Error consulta de Usuarios", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub ConsultaUsrAdmin(ByVal user As String)
        Dim dt As New DataTable()
        Try
            If checkSvrOnline(ServerSQL) = True Then
                pendiente.DataGridView1.DataSource = Nothing
                pendiente.DataGridView1.Refresh()
                pendiente.DataGridView1.Rows.Clear()
                Dim sql_command As String = "SELECT * FROM PackoutAdminIMX WHERE No = '" & user & "'"
                Dim adapter As New SqlDataAdapter(sql_command, con)
                adapter.Fill(dt)
                pendiente.DataGridView1.DataSource = dt
            End If
        Catch ex As Exception
            Sqll.Err("Error consulta de Usuarios", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub insrt(ByVal operador As String, ByVal operadoradmin As String, ByVal comentario As String)
        Dim dia As String = DateTime.Now.ToString("yyyy/MM/dd")
        Dim hora As String = DateTime.Now.ToString("HH:mm:ss")
        Dim fechahora As String = dia & " " & hora
        Beep()
        Try
            If checkSvrOnline(ServerSQL) = True Then
                If con.State = 1 Then
                    con.Close()
                End If
                con.Open()
                cmd.Connection = con
                cmd.CommandText = "INSERT INTO PackoutResultadosIMX (FechaHora, Pedido, Serie, Resultado, Operador, OperadorAdmin, Comentario) VALUES('" _
                & fechahora & "', '" _
                & Form1.lblPedido.Text & "', '" _
                & Form1.lblSerie.Text & "', '" _
                & Form1.res & "' , '" _ 'Form1.lblRes.Text & "', '" _
                & operador & "', '" _
                & operadoradmin & "', '" _
                & comentario & "')"
                cmd.ExecuteNonQuery()
            End If
        Catch ex As Exception
            Sqll.Err("Error insertar Resultados", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        Finally
            con.Close()
        End Try
    End Sub
    Public Sub ConsultaUltimosSeriesPackout(ByVal cb As ComboBox)
        Dim dr As SqlDataReader
        Try
            If checkSvrOnline(ServerSQL) = True Then
                If cb.Items.Count > 0 Then
                    cb.Items.Clear()
                End If
                If con.State = ConnectionState.Open Then
                    con.Close()
                End If
                con.Open()
                cmd.Connection = con
                cmd.CommandText = "SELECT TOP 20 * FROM PackoutResViewIMX WHERE Resultado = 'APROBADO' ORDER BY FechaHora DESC" '"SELECT TOP 10 DISTINCT * FROM MAC"
                dr = cmd.ExecuteReader
                Do While dr.Read = True
                    cb.Items.Add(dr.GetString(2))
                Loop
            End If
        Catch ex As Exception
            'MsgBox("Error ConsultaUltimosSeriesPackout", ex.ToString, True)
            ErrorLog.logg(ex.Message, ex.ToString)
        Finally
            con.Close()
        End Try
    End Sub
    Public Function checkSvrOnline(ByVal servidor As String) As Boolean
        Dim res As Boolean = False
        Try
            Dim _ping As New Ping
            Dim _pingreply = _ping.Send(servidor, 2000)
            If _pingreply.Status = IPStatus.Success Then
                'server is online
                res = True
                'Form1.BackColor = Color.DodgerBlue
                Form1.lblSQL.Text = "SQL OK"
            Else
                'server is offline
                res = False
                'Form1.BackColor = Color.DarkRed
                Form1.lblSQL.Text = "SQL Not OK"
            End If
        Catch ex As Exception
            'MsgBox("Error Servidor SQL", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
        Return res
    End Function
    Public Sub ConsultaUsrAdminLogin(ByVal user As String)
        Dim dt As New DataTable()
        Try
            If checkSvrOnline(ServerSQL) = True Then
                LoginForm.dtgLogin.DataSource = Nothing
                LoginForm.dtgLogin.Refresh()
                LoginForm.dtgLogin.Rows.Clear()
                Dim sql_command As String = "SELECT * FROM PackoutAdminIMX WHERE No = '" & user & "'"
                Dim adapter As New SqlDataAdapter(sql_command, con)
                adapter.Fill(dt)
                LoginForm.dtgLogin.DataSource = dt
            End If
        Catch ex As Exception
            'Sqll.Err("Error consulta de Usuarios", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
End Module