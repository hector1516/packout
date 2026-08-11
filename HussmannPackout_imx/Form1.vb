Imports System.Deployment.Application
Imports System.Drawing.Text
Public Class Form1
    Public ver As String
    Public lbl_items(100) As Label
    Public listitems As List(Of String)
    Dim errC As Integer = 0
    Public vali1, vali2, vali3 As Boolean
    Public descPendiente As String
    Public itemss(100) As String
    Dim band As Integer = 0
    Public localizacion As String
    Public LoginUsr As Boolean = False
    Public Obs As String = ""
    Public res As String = ""
    Private Sub Form1_Load(sender As Object, e As EventArgs) Handles MyBase.Load
        If ApplicationDeployment.IsNetworkDeployed Then
            ver = ApplicationDeployment.CurrentDeployment.CurrentVersion.ToString
        Else
            ver = Application.ProductVersion.ToString
        End If
        If CheckBoxMapicsServer.Checked = True Then
            mapics.MapicsStringcon = "DSN=test;UID=DATRINS;PWD=PRUEBAS"
        Else
            mapics.MapicsStringcon = "DSN=datatest;UID=DATRINS;PWD=DATA18"
        End If
        Me.Text = "Packout - " & ver
        SeleccionImpresora()
        localizacion = SelImpresora(System.Windows.Forms.SystemInformation.ComputerName)
        lblPCName.Text = localizacion
        txtRawScanner.Focus()
        txtRawScanner.Select()
        Sqll.ConsultaResultados()
        'create_labels(lbl_items, PanelItems, 100, 10, 10, 400)
        Me.KeyPreview = True
        lblSerie.Text = ""
        lblPedido.Text = ""
        'lblRes.Text = ""
        'lblScanNext.Text = ""
        vali1 = False
        vali2 = False
        vali3 = False
        Sqll.Err("inicio de la aplicacion", "inicio de la aplicacion en: " & ver & " " & localizacion)
        Sqll.Err("Conexion SQL", Sqll.sqlstringcon)
        Sqll.Err("Conexion Mapics", mapics.MapicsStringcon)
        InstallUpdateSyncWithInfo()
    End Sub
    Private Sub Timer1_Tick(sender As Object, e As EventArgs) Handles Timer1.Tick
        lblFecha.Text = DateTime.Now.ToString("dd/MM/yyyy")
        lblHora.Text = DateTime.Now.ToString("HH:mm:ss")
        errC = errC + 1
        lblItems.Text = band
        If errC = 60 Then
            errC = 0
            lblError.Text = "Sin errores recientes"
            Sqll.ConsultaResultados()
        End If
    End Sub
    Private Sub Form1_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Enter Then
            If txtRawScanner.Text.Length > 0 Then
                ValidateSN()
                txtRawScanner.SelectAll()
            End If
        End If
    End Sub
    Public Sub ValidateSN()
        Try
            Dim vartexto As String
            Sqll.ConsultaUsr("xx")
            descPendiente = ""
            If vali1 = True And vali2 = True And vali3 = True Then
                If txtRawScanner2.TextLength > 6 Then
                    If lblSerie.Text.Length > 2 Then
                        If lblPedido.Text.Length > 2 Then
                            If vali3 = True Then
                                Sqll.ConsultaUsr(txtRawScanner2.Text)
                                If dtgUsers.Rows.Count > 0 Then
                                    'lblBatchInd.ForeColor = Color.Green
                                    Sqll.insrt(txtRawScanner2.Text, "N/A", Obs)
                                    Sqll.ConsultaResultados()
                                    vali1 = False
                                    vali2 = False
                                    vali3 = False
                                    lblSerie.Text = ""
                                    lblPedido.Text = ""
                                    'lblRes.Text = ""
                                    'lblItemsInd.ForeColor = Color.Black
                                    'lblBatchInd.ForeColor = Color.Black
                                    'lblNoSerieInd.ForeColor = Color.Black
                                    For i = 0 To 99
                                        lbl_items(i).Text = ""
                                    Next
                                    txtRawScanner2.Text = ""
                                    txtRawScanner2.SelectAll()
                                    txtRawScanner.Text = ""
                                    txtRawScanner.SelectAll()
                                    txtRawScanner.Visible = False
                                    Application.Restart()
                                End If
                            End If
                        End If
                    End If
                End If
            ElseIf vali1 = True And vali2 = True And vali3 = False Then
                Dim val As Integer = dtgMapics.Rows.Count - 1
                If txtRawScanner2.TextLength > 2 Then
                    For i = 0 To dtgItems.Rows.Count - 1
                        If String.Compare(txtRawScanner2.Text.ToUpper, dtgItems.Item(0, i).Value.ToString) = 0 Then
                            dtgItems.Rows.RemoveAt(i)
                            band = band - 1
                            fotosItems(i)
                            txtRawScanner.Visible = False
                        End If
                    Next
                    If band = 0 Then
                        If dtgItems.Rows.Count = 0 Then
                            vali3 = True
                            mapics.insert(lblSerie.Text)
                            res = "APROBADO"
                            'lblRes.Text = "APROBADO"
                            'lblRes.ForeColor = Color.Green
                            'lblItemsInd.ForeColor = Color.Green
                            txtRawScanner.Visible = False
                        End If
                    ElseIf band > 0 Then
                        vali3 = False
                        res = "PENDIENTE"
                        'lblRes.Text = "PENDIENTE"
                        'lblRes.ForeColor = Color.Red
                        'lblItemsInd.ForeColor = Color.Black
                        txtRawScanner.Visible = False
                    End If
                End If
            ElseIf vali1 = False And vali2 = False And vali3 = False Then
                If txtRawScanner2.TextLength > 2 Then
                    vartexto = txtRawScanner2.Text
                    Dim seriee As String = vartexto.Substring(0, 1)
                    If String.Compare("MY", seriee) Or String.Compare("my", seriee) Then
                        If vartexto.Contains("MY") Or vartexto.Contains("my") Then
                            lblSerie.Text = ""
                            lblSerie.Text = vartexto.Replace(" ", "").ToUpper
                            'lblNoSerieInd.ForeColor = Color.Green
                            vali1 = True
                            mapics.Consulta(lblSerie.Text)
                            txtRawScanner.Visible = False
                            If dtgMapics.Rows.Count > 0 Then
                                lblPedido.Text = dtgMapics.Item(2, 0).Value
                                createlabels()
                                vali2 = True
                                res = "PENDIENTE"
                                'lblRes.Text = "PENDIENTE"
                                'lblRes.ForeColor = Color.Red
                                txtRawScanner.Visible = False
                            Else
                                lblPedido.Text = ""
                                vali2 = False
                                vali1 = False
                                lblSerie.Text = "N/A"
                                'lblNoSerieInd.ForeColor = Color.Black
                                Sqll.Err("Sin items", "Sin item encontrados en mapics con el numero de serie: " & txtRawScanner2.Text)
                                txtRawScanner.Visible = False
                            End If
                        Else
                            lblSerie.Text = "N/A"
                            'lblNoSerieInd.ForeColor = Color.Black
                            vali1 = False
                            txtRawScanner.Visible = False
                        End If
                    Else
                        lblSerie.Text = "N/A"
                        'lblNoSerieInd.ForeColor = Color.Black
                        vali1 = False
                        txtRawScanner.Visible = False
                    End If
                End If
            End If
            txtRawScanner.Visible = False
        Catch ex As Exception
        End Try
    End Sub
    Private Sub TxtRawScanner_TextChanged(sender As Object, e As EventArgs) Handles txtRawScanner.TextChanged
        lblescanner.Text = txtRawScanner.Text
        txtRawScanner2.Text = txtRawScanner.Text.Replace("(", "")
    End Sub
    Private Sub txtRawScanner_Leave(sender As Object, e As EventArgs) Handles txtRawScanner.Leave
        txtRawScanner.BackColor = Color.Red
        txtRawScanner.Focus()
        txtRawScanner.Select()
    End Sub
    Private Sub txtRawScanner_LostFocus(sender As Object, e As EventArgs) Handles txtRawScanner.LostFocus
        txtRawScanner.BackColor = Color.Red
        txtRawScanner.Focus()
        txtRawScanner.Select()
    End Sub
    Private Sub txtRawScanner_GotFocus(sender As Object, e As EventArgs) Handles txtRawScanner.GotFocus
        txtRawScanner.BackColor = Color.DodgerBlue
        txtRawScanner.ForeColor = Color.White
    End Sub
    Public Sub createlabels()
        Try
            If dtgItems.Rows.Count > 0 Then
                dtgItems.Rows.Clear()
            End If
            For i = 0 To dtgMapics.Rows.Count - 1
                If String.Compare(dtgMapics.Item(12, i).Value.ToString, "Disabled") = 0 Then
                Else
                    If String.Compare(dtgMapics.Item(5, i).Value.ToString, "Y") = 0 Then
                        If dtgMapics.Item(10, i).Value.ToString.Contains("A") Then
                            band = band + 1
                            If dtgMapics.Item(8, i).Value.ToString.Length > 1 Then
                                If String.Compare(dtgMapics.Item(0, i).Value.ToString, "90") = 0 Or String.Compare(dtgMapics.Item(0, i).Value.ToString, "790") = 0 Or String.Compare(dtgMapics.Item(0, i).Value.ToString, "810") = 0 Or String.Compare(dtgMapics.Item(0, i).Value.ToString, "910") = 0 Then
                                    itemss(i) = lblSerie.Text & dtgMapics.Item(10, i).Value.ToString
                                    dtgItems.Rows.Add(dtgMapics.Item(3, i).Value.ToString.Replace(" ", "") & dtgMapics.Item(10, i).Value.ToString.Replace(" ", ""), dtgMapics.Item(4, i).Value.ToString.Replace(" ", ""))
                                    lbl_items(i).Text = lblSerie.Text & dtgMapics.Item(10, i).Value.ToString & " - " & dtgMapics.Item(4, i).Value.ToString '"Item " & indice.ToString.PadLeft(2, "0"c)
                                Else
                                    dtgItems.Rows.Add(dtgMapics.Item(3, i).Value.ToString.Replace(" ", "") & "XX1", dtgMapics.Item(4, i).Value.ToString.Replace(" ", ""))
                                End If
                            Else
                            End If
                        Else
                        End If
                    Else
                        dtgItems.Rows.Add(dtgMapics.Item(3, i).Value.ToString.Replace(" ", "") & "XX3", dtgMapics.Item(4, i).Value.ToString.Replace(" ", ""))
                    End If
                End If
            Next
            'GroupBoxItems.Text = "Items " & dtgMapics.Rows.Count
        Catch ex As Exception
            Sqll.Err("Error al llenar labels en form", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Private Sub Button1_Click(sender As Object, e As EventArgs) Handles Button1.Click
        Button1.Enabled = False
        If LoginUsr = True Then
            Button1.ForeColor = Color.Black
            Me.WindowState = FormWindowState.Minimized
            pendiente.Show()
            LoginUsr = False
        Else
            Button1.ForeColor = Color.Red
        End If
        Button1.Enabled = True
    End Sub
    Public Sub create_labels(ByVal nomlabel As Label(), ByVal lugar As Control, ByVal cantidad As Integer, ByVal xx As Integer, ByVal yy As Integer, ByVal largo As Integer)
        Dim cnt As Integer
        Dim myfont As New Font("Tahoma", 15.75, FontStyle.Bold)
        Try
            For i = 0 To cantidad - 1
                Dim loc As New Point(xx, cnt + yy)
                nomlabel(i) = New Label
                nomlabel(i).Name = nomlabel(i).Name.ToString & "(" & i & ")"
                nomlabel(i).Size = New Size(largo, 21)
                nomlabel(i).Location = loc
                nomlabel(i).BackColor = Color.Transparent
                nomlabel(i).BorderStyle = BorderStyle.None
                nomlabel(i).TextAlign = ContentAlignment.MiddleLeft
                nomlabel(i).Font = myfont
                cnt = cnt + (nomlabel(i).Height + 3)
            Next
            lugar.Controls.AddRange(nomlabel)
        Catch ex As Exception
            Sqll.Err("Error al crear labels en form", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub Temporada()
        Try
            Dim privateFont As New System.Drawing.Text.PrivateFontCollection
            Dim mes As String = DateTime.Now.ToString("MM")
            privateFont.AddFontFile(".\" & mes & ".ttf")
            Dim font48 As New System.Drawing.Font(privateFont.Families(0), 48)
            Dim font24 As New System.Drawing.Font(privateFont.Families(0), 24)
            Dim font160 As New System.Drawing.Font(privateFont.Families(0), 160)
            Dim font21 As New System.Drawing.Font(privateFont.Families(0), 21)
            lblSerie.Font = font48
            lblPedido.Font = font48
            'lblRes.Font = font48
            'lblScanNext.Font = font48
        Catch ex As Exception
            Sqll.Err("Error modulo temporada", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Public Sub fotosItems(ByVal index As Integer)
        Try
            If System.IO.File.Exists("C:\ECCSA\HUSS\PackOut\Items\" & dtgItems.Item(1, index).Value.ToString & ".png") Then
                'pbItem.BackgroundImage = Image.FromFile("C:\ECCSA\HUSS\PackOut\Items\" & dtgItems.Item(1, index).Value.ToString & ".png")
            Else
                'pbItem.BackgroundImage = Image.FromFile("C:\ECCSA\HUSS\PackOut\Items\NA.png")
            End If
        Catch ex As Exception
            Sqll.Err("Error modulo foto item", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
    Private Sub Form1_KeyPress(sender As Object, e As KeyPressEventArgs) Handles Me.KeyPress
        If Asc(e.KeyChar) = 40 Then '(
            e.Handled = True
            txtRawScanner.Visible = True
            txtRawScanner.Focus()
        Else
            e.Handled = False
        End If
    End Sub
    Private Sub CheckBoxMapicsServer_CheckedChanged(sender As Object, e As EventArgs) Handles CheckBoxMapicsServer.CheckedChanged
        If CheckBoxMapicsServer.Checked = True Then
            mapics.MapicsStringcon = "DSN=test;UID=DATRINS;PWD=PRUEBAS"
            mapics.MapicsServer = "test.hussmann.com"
        Else
            mapics.MapicsStringcon = "DSN=datatest;UID=DATRINS;PWD=DATA18"
            mapics.MapicsServer = "prod.hussmann.com"
        End If
        Sqll.Err("Servidor de Mapics", mapics.MapicsStringcon & " Server: " & mapics.MapicsServer)
    End Sub
    Private Sub InstallUpdateSyncWithInfo()
        Dim info As UpdateCheckInfo = Nothing
        If (ApplicationDeployment.IsNetworkDeployed) Then
            Dim AD As ApplicationDeployment = ApplicationDeployment.CurrentDeployment
            Try
                info = AD.CheckForDetailedUpdate()
            Catch dde As DeploymentDownloadException
                MessageBox.Show("La nueva version de la aplicacion no se puede descargar del servidor. " + ControlChars.Lf & ControlChars.Lf & "Revisa tu conexion a internet o intenta mas tarde. Error: " + dde.Message)
                Return
            Catch ioe As InvalidOperationException
                'MessageBox.Show("La aplicacion no puede actualizarse. Error: " & ioe.Message)
                ErrorLog.logg(ioe.Message, ioe.ToString)
                Return
            End Try
            If (info.UpdateAvailable) Then
                Dim doUpdate As Boolean = True
                If (Not info.IsUpdateRequired) Then
                    Dim dr As DialogResult = MessageBox.Show("Hay una nueva actualizacion disponible. Deseas actualizar la aplicacion ahora?", "Actualizacion Disponible", MessageBoxButtons.OKCancel)
                    If (Not System.Windows.Forms.DialogResult.OK = dr) Then
                        doUpdate = False
                    End If
                Else
                    ' Display a message that the app MUST reboot. Display the minimum required version.
                    MessageBox.Show("This application has detected a mandatory update from your current " &
                        "version to version " & info.MinimumRequiredVersion.ToString() &
                        ". The application will now install the update and restart.",
                        "Update Available", MessageBoxButtons.OK,
                        MessageBoxIcon.Information)
                End If
                If (doUpdate) Then
                    Try
                        AD.Update()
                        MessageBox.Show("La aplicacion se a actualizado, es necesario reiniciarla.")
                        Application.Restart()
                    Catch dde As DeploymentDownloadException
                        'MessageBox.Show("Algo paso que no se pudo instalar la actualizacion. " & ControlChars.Lf & ControlChars.Lf & "Parece que es debido a tu conexion a internet, o podrias intentarlo despues.")
                        ErrorLog.logg(dde.Message, dde.ToString)
                        Return
                    End Try
                End If
            End If
        End If
    End Sub
    Private Sub btnReimprimir_Click(sender As Object, e As EventArgs) Handles btnReimprimir.Click
        btnReimprimir.Enabled = False
        If LoginUsr = True Then
            btnReimprimir.ForeColor = Color.Black
            FormReimprimir.Show()
            LoginUsr = False
        Else
            btnReimprimir.ForeColor = Color.Red
        End If
        btnReimprimir.Enabled = True
    End Sub
    Private Sub PictureBox1_DoubleClick(sender As Object, e As EventArgs)
        InstallUpdateSyncWithInfo()
    End Sub
    Private Sub btnPruebas_Click(sender As Object, e As EventArgs) Handles btnPruebas.Click
        btnPruebas.Enabled = True
        Pruebas.Show()
        btnPruebas.Enabled = False
    End Sub
    Private Sub btnLogin_Click(sender As Object, e As EventArgs) Handles btnLogin.Click
        btnLogin.Enabled = False
        LoginForm.Show()
        btnLogin.Enabled = True
    End Sub
    Private Sub btnManual_Click(sender As Object, e As EventArgs) Handles btnManual.Click
        btnManual.Enabled = False
        If LoginUsr = True Then
            btnManual.ForeColor = Color.Black
            Me.WindowState = FormWindowState.Minimized
            Manual.Show()
            LoginUsr = False
        Else
            btnManual.ForeColor = Color.Red
        End If
        btnManual.Enabled = True
    End Sub
    Private Sub Button2_Click(sender As Object, e As EventArgs) Handles Button2.Click
        Button2.Enabled = False
        FormImagenes.Show()
        Button2.Enabled = True
    End Sub
    Public Function SelImpresora(ByVal pcname As String) As String
        Dim res As String = ""
        Try
            If String.Compare(pcname, "MRYD-37W0H63") = 0 Then
                res = "ESTPACK01"
                Sqll.Err("Identidicacion de equipo", "Nombre del equipo: " & pcname & " Estacion Hussmann XACHGMEP/ECCSAPRT: " & res)
            End If
            If String.Compare(pcname, "MRYD-1NZPMN2") = 0 Then
                res = "ESTPACK02"
                Sqll.Err("Identidicacion de equipo", "Nombre del equipo: " & pcname & " Estacion Hussmann XACHGMEP/ECCSAPRT: " & res)
            End If
            If String.Compare(pcname, "MRYL-DMQQ2D3") = 0 Then
                res = "ESTPACK01"
                Sqll.Err("Identidicacion de equipo", "Nombre del equipo: " & pcname & " Estacion Hussmann XACHGMEP/ECCSAPRT: " & res)
            End If
            If res.Length < 1 Then
                res = "ESTPACK01"
                Sqll.Err("Identidicacion de equipo", "Nombre del equipo: " & pcname & " Estacion No encontrada en Hussmann XACHGMEP/ECCSAPRT")
            End If
        Catch ex As Exception
            Sqll.Err("SelImpresora", ex.ToString)
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
        Return res
    End Function
    Public Sub SeleccionImpresora()
        Dim res As String
        Dim pcname As String = System.Windows.Forms.SystemInformation.ComputerName
        Try
            If String.Compare(pcname, "MRYD-37W0H63") = 0 Then
                res = "ESTPACK01"
                mapics.MapicsQuery = "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE', 'ESTPACK01', CURRENT DATE, CURRENT TIME, 'ECCSA' FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '"
                Sqll.Err("Identidicacion de equipo", "Nombre del equipo: " & pcname & " Estacion Hussmann XACHGMEP/ECCSAPRT: " & res)
            End If
            If String.Compare(pcname, "MRYD-1NZPMN2") = 0 Then
                res = "ESTPACK02"
                mapics.MapicsQuery = "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE', 'ESTPACK02', CURRENT DATE, CURRENT TIME, 'ECCSA' FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '"
                Sqll.Err("Identidicacion de equipo", "Nombre del equipo: " & pcname & " Estacion Hussmann XACHGMEP/ECCSAPRT: " & res)
            Else
                res = "N/A"
                mapics.MapicsQuery = "INSERT INTO XACHGMEP.FESRLKIT SELECT Distinct C.INSLIN, C.INSPED, C.INSNEQ, C.INSITE,C.INSMOO,C.INSSER,C.INSSCU ,'PACKOUT COMPLETE', 'ESTPACK01', CURRENT DATE, CURRENT TIME, 'ECCSA' FROM XACHGMEP.EPCIMAGE A INNER JOIN XACHGMEP.EPCBITA B on A.IMANUSE = B.EPCSERIE AND A.IMAKIT = B.EPCKIT INNER JOIN XACHGMEP.BAN100PF C ON A.IMANUSE = C.INSSER WHERE A.IMANUSE = '"
                Sqll.Err("Sin Identidicacion de equipo", "Nombre del equipo: " & pcname & " Estacion Hussmann XACHGMEP/ECCSAPRT: " & res)
            End If
        Catch ex As Exception
            ErrorLog.logg(ex.Message, ex.ToString)
        End Try
    End Sub
End Class