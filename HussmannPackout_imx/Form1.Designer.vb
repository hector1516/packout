<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()>
Partial Class Form1
    Inherits System.Windows.Forms.Form

    'Form overrides dispose to clean up the component list.
    <System.Diagnostics.DebuggerNonUserCode()>
    Protected Overrides Sub Dispose(ByVal disposing As Boolean)
        Try
            If disposing AndAlso components IsNot Nothing Then
                components.Dispose()
            End If
        Finally
            MyBase.Dispose(disposing)
        End Try
    End Sub

    'Required by the Windows Form Designer
    Private components As System.ComponentModel.IContainer

    'NOTE: The following procedure is required by the Windows Form Designer
    'It can be modified using the Windows Form Designer.  
    'Do not modify it using the code editor.
    <System.Diagnostics.DebuggerStepThrough()>
    Private Sub InitializeComponent()
        Me.components = New System.ComponentModel.Container()
        Dim DataGridViewCellStyle1 As System.Windows.Forms.DataGridViewCellStyle = New System.Windows.Forms.DataGridViewCellStyle()
        Dim DataGridViewCellStyle2 As System.Windows.Forms.DataGridViewCellStyle = New System.Windows.Forms.DataGridViewCellStyle()
        Dim DataGridViewCellStyle3 As System.Windows.Forms.DataGridViewCellStyle = New System.Windows.Forms.DataGridViewCellStyle()
        Dim DataGridViewCellStyle4 As System.Windows.Forms.DataGridViewCellStyle = New System.Windows.Forms.DataGridViewCellStyle()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(Form1))
        Me.StatusStrip1 = New System.Windows.Forms.StatusStrip()
        Me.lblFecha = New System.Windows.Forms.ToolStripStatusLabel()
        Me.lblHora = New System.Windows.Forms.ToolStripStatusLabel()
        Me.lblError = New System.Windows.Forms.ToolStripStatusLabel()
        Me.lblUserLogin = New System.Windows.Forms.ToolStripStatusLabel()
        Me.lblSQL = New System.Windows.Forms.ToolStripStatusLabel()
        Me.lblItems = New System.Windows.Forms.ToolStripStatusLabel()
        Me.lblescanner = New System.Windows.Forms.ToolStripStatusLabel()
        Me.Timer1 = New System.Windows.Forms.Timer(Me.components)
        Me.flpImagenes = New System.Windows.Forms.FlowLayoutPanel()
        Me.lblPCName = New System.Windows.Forms.Label()
        Me.lblPedido = New System.Windows.Forms.Label()
        Me.lblSerie = New System.Windows.Forms.Label()
        Me.dtgUsers = New System.Windows.Forms.DataGridView()
        Me.txtRawScanner = New System.Windows.Forms.TextBox()
        Me.dtgItems = New System.Windows.Forms.DataGridView()
        Me.ColumnSerie = New System.Windows.Forms.DataGridViewTextBoxColumn()
        Me.itemsC = New System.Windows.Forms.DataGridViewTextBoxColumn()
        Me.TabControl1 = New System.Windows.Forms.TabControl()
        Me.TabPage5 = New System.Windows.Forms.TabPage()
        Me.TableLayoutPanel1 = New System.Windows.Forms.TableLayoutPanel()
        Me.FlowLayoutPanel1 = New System.Windows.Forms.FlowLayoutPanel()
        Me.Button2 = New System.Windows.Forms.Button()
        Me.btnLogin = New System.Windows.Forms.Button()
        Me.btnReimprimir = New System.Windows.Forms.Button()
        Me.btnManual = New System.Windows.Forms.Button()
        Me.Button1 = New System.Windows.Forms.Button()
        Me.TabPage1 = New System.Windows.Forms.TabPage()
        Me.btnPruebas = New System.Windows.Forms.Button()
        Me.CheckBoxMapicsServer = New System.Windows.Forms.CheckBox()
        Me.dtgMapics = New System.Windows.Forms.DataGridView()
        Me.dtg_error = New System.Windows.Forms.DataGridView()
        Me.ColumnFechaHora = New System.Windows.Forms.DataGridViewTextBoxColumn()
        Me.ColumnTitulo = New System.Windows.Forms.DataGridViewTextBoxColumn()
        Me.ColumnDescripcion = New System.Windows.Forms.DataGridViewTextBoxColumn()
        Me.dtgHistorico = New System.Windows.Forms.DataGridView()
        Me.txtRawScanner2 = New System.Windows.Forms.TextBox()
        Me.StatusStrip1.SuspendLayout()
        CType(Me.dtgUsers, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.dtgItems, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.TabControl1.SuspendLayout()
        Me.TabPage5.SuspendLayout()
        Me.TableLayoutPanel1.SuspendLayout()
        Me.FlowLayoutPanel1.SuspendLayout()
        Me.TabPage1.SuspendLayout()
        CType(Me.dtgMapics, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.dtg_error, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.dtgHistorico, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'StatusStrip1
        '
        Me.StatusStrip1.Items.AddRange(New System.Windows.Forms.ToolStripItem() {Me.lblFecha, Me.lblHora, Me.lblError, Me.lblUserLogin, Me.lblSQL, Me.lblItems, Me.lblescanner})
        Me.StatusStrip1.Location = New System.Drawing.Point(0, 971)
        Me.StatusStrip1.Name = "StatusStrip1"
        Me.StatusStrip1.Size = New System.Drawing.Size(1904, 22)
        Me.StatusStrip1.TabIndex = 0
        Me.StatusStrip1.Text = "StatusStrip1"
        '
        'lblFecha
        '
        Me.lblFecha.ActiveLinkColor = System.Drawing.Color.White
        Me.lblFecha.Name = "lblFecha"
        Me.lblFecha.Size = New System.Drawing.Size(65, 17)
        Me.lblFecha.Text = "00/00/0000"
        '
        'lblHora
        '
        Me.lblHora.ActiveLinkColor = System.Drawing.Color.White
        Me.lblHora.Name = "lblHora"
        Me.lblHora.Size = New System.Drawing.Size(49, 17)
        Me.lblHora.Text = "00:00:00"
        '
        'lblError
        '
        Me.lblError.ActiveLinkColor = System.Drawing.Color.White
        Me.lblError.Name = "lblError"
        Me.lblError.Size = New System.Drawing.Size(112, 17)
        Me.lblError.Text = "Sin errores recientes"
        '
        'lblUserLogin
        '
        Me.lblUserLogin.ActiveLinkColor = System.Drawing.Color.White
        Me.lblUserLogin.Name = "lblUserLogin"
        Me.lblUserLogin.Size = New System.Drawing.Size(60, 17)
        Me.lblUserLogin.Text = "LoginUser"
        '
        'lblSQL
        '
        Me.lblSQL.Name = "lblSQL"
        Me.lblSQL.Size = New System.Drawing.Size(47, 17)
        Me.lblSQL.Text = "SQL OK"
        '
        'lblItems
        '
        Me.lblItems.Name = "lblItems"
        Me.lblItems.Size = New System.Drawing.Size(19, 17)
        Me.lblItems.Text = "00"
        '
        'lblescanner
        '
        Me.lblescanner.Name = "lblescanner"
        Me.lblescanner.Size = New System.Drawing.Size(31, 17)
        Me.lblescanner.Text = "0000"
        '
        'Timer1
        '
        Me.Timer1.Enabled = True
        Me.Timer1.Interval = 1000
        '
        'flpImagenes
        '
        Me.flpImagenes.Anchor = CType((((System.Windows.Forms.AnchorStyles.Top Or System.Windows.Forms.AnchorStyles.Bottom) _
            Or System.Windows.Forms.AnchorStyles.Left) _
            Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.flpImagenes.AutoScroll = True
        Me.flpImagenes.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.flpImagenes.Location = New System.Drawing.Point(8, 6)
        Me.flpImagenes.Name = "flpImagenes"
        Me.flpImagenes.Size = New System.Drawing.Size(1868, 854)
        Me.flpImagenes.TabIndex = 1612
        '
        'lblPCName
        '
        Me.lblPCName.Anchor = CType((System.Windows.Forms.AnchorStyles.Top Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.lblPCName.BackColor = System.Drawing.Color.Transparent
        Me.lblPCName.Font = New System.Drawing.Font("Tahoma", 8.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.lblPCName.Location = New System.Drawing.Point(1220, 904)
        Me.lblPCName.Name = "lblPCName"
        Me.lblPCName.Size = New System.Drawing.Size(128, 17)
        Me.lblPCName.TabIndex = 1611
        Me.lblPCName.Text = "PC Name"
        Me.lblPCName.TextAlign = System.Drawing.ContentAlignment.MiddleLeft
        '
        'lblPedido
        '
        Me.lblPedido.BackColor = System.Drawing.Color.Transparent
        Me.lblPedido.Dock = System.Windows.Forms.DockStyle.Fill
        Me.lblPedido.Font = New System.Drawing.Font("Tahoma", 38.25!, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.lblPedido.Location = New System.Drawing.Point(1310, 0)
        Me.lblPedido.Name = "lblPedido"
        Me.lblPedido.Size = New System.Drawing.Size(565, 61)
        Me.lblPedido.TabIndex = 1605
        Me.lblPedido.Text = "XXXXXXXX"
        Me.lblPedido.TextAlign = System.Drawing.ContentAlignment.MiddleCenter
        '
        'lblSerie
        '
        Me.lblSerie.BackColor = System.Drawing.Color.Transparent
        Me.lblSerie.Dock = System.Windows.Forms.DockStyle.Fill
        Me.lblSerie.Font = New System.Drawing.Font("Tahoma", 38.25!, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.lblSerie.Location = New System.Drawing.Point(3, 0)
        Me.lblSerie.Name = "lblSerie"
        Me.lblSerie.Size = New System.Drawing.Size(564, 61)
        Me.lblSerie.TabIndex = 1604
        Me.lblSerie.Text = "MYXXXXXX"
        Me.lblSerie.TextAlign = System.Drawing.ContentAlignment.MiddleCenter
        '
        'dtgUsers
        '
        Me.dtgUsers.AllowUserToAddRows = False
        Me.dtgUsers.AllowUserToDeleteRows = False
        Me.dtgUsers.AllowUserToResizeColumns = False
        Me.dtgUsers.AllowUserToResizeRows = False
        Me.dtgUsers.AutoSizeColumnsMode = System.Windows.Forms.DataGridViewAutoSizeColumnsMode.AllCells
        Me.dtgUsers.AutoSizeRowsMode = System.Windows.Forms.DataGridViewAutoSizeRowsMode.AllCells
        Me.dtgUsers.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize
        Me.dtgUsers.Location = New System.Drawing.Point(1223, 416)
        Me.dtgUsers.Name = "dtgUsers"
        Me.dtgUsers.Size = New System.Drawing.Size(665, 483)
        Me.dtgUsers.TabIndex = 1608
        '
        'txtRawScanner
        '
        Me.txtRawScanner.Anchor = CType((System.Windows.Forms.AnchorStyles.Bottom Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.txtRawScanner.CharacterCasing = System.Windows.Forms.CharacterCasing.Upper
        Me.txtRawScanner.Font = New System.Drawing.Font("Tahoma", 6.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.txtRawScanner.Location = New System.Drawing.Point(1354, 974)
        Me.txtRawScanner.Name = "txtRawScanner"
        Me.txtRawScanner.Size = New System.Drawing.Size(261, 17)
        Me.txtRawScanner.TabIndex = 1
        Me.txtRawScanner.TabStop = False
        Me.txtRawScanner.TextAlign = System.Windows.Forms.HorizontalAlignment.Center
        Me.txtRawScanner.Visible = False
        '
        'dtgItems
        '
        Me.dtgItems.AllowUserToAddRows = False
        Me.dtgItems.AllowUserToDeleteRows = False
        Me.dtgItems.AllowUserToOrderColumns = True
        Me.dtgItems.AllowUserToResizeColumns = False
        Me.dtgItems.AllowUserToResizeRows = False
        Me.dtgItems.BackgroundColor = System.Drawing.Color.White
        Me.dtgItems.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize
        Me.dtgItems.Columns.AddRange(New System.Windows.Forms.DataGridViewColumn() {Me.ColumnSerie, Me.itemsC})
        Me.dtgItems.Location = New System.Drawing.Point(614, 412)
        Me.dtgItems.Name = "dtgItems"
        Me.dtgItems.Size = New System.Drawing.Size(600, 510)
        Me.dtgItems.TabIndex = 1611
        '
        'ColumnSerie
        '
        Me.ColumnSerie.AutoSizeMode = System.Windows.Forms.DataGridViewAutoSizeColumnMode.AllCells
        Me.ColumnSerie.HeaderText = "Serials"
        Me.ColumnSerie.Name = "ColumnSerie"
        Me.ColumnSerie.ReadOnly = True
        Me.ColumnSerie.Width = 63
        '
        'itemsC
        '
        Me.itemsC.AutoSizeMode = System.Windows.Forms.DataGridViewAutoSizeColumnMode.Fill
        Me.itemsC.HeaderText = "Items"
        Me.itemsC.Name = "itemsC"
        Me.itemsC.ReadOnly = True
        '
        'TabControl1
        '
        Me.TabControl1.Anchor = CType((((System.Windows.Forms.AnchorStyles.Top Or System.Windows.Forms.AnchorStyles.Bottom) _
            Or System.Windows.Forms.AnchorStyles.Left) _
            Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.TabControl1.Controls.Add(Me.TabPage5)
        Me.TabControl1.Controls.Add(Me.TabPage1)
        Me.TabControl1.Location = New System.Drawing.Point(12, 12)
        Me.TabControl1.Multiline = True
        Me.TabControl1.Name = "TabControl1"
        Me.TabControl1.SelectedIndex = 0
        Me.TabControl1.Size = New System.Drawing.Size(1892, 956)
        Me.TabControl1.SizeMode = System.Windows.Forms.TabSizeMode.Fixed
        Me.TabControl1.TabIndex = 1611
        '
        'TabPage5
        '
        Me.TabPage5.Controls.Add(Me.TableLayoutPanel1)
        Me.TabPage5.Controls.Add(Me.flpImagenes)
        Me.TabPage5.Location = New System.Drawing.Point(4, 22)
        Me.TabPage5.Name = "TabPage5"
        Me.TabPage5.Padding = New System.Windows.Forms.Padding(3)
        Me.TabPage5.Size = New System.Drawing.Size(1884, 930)
        Me.TabPage5.TabIndex = 4
        Me.TabPage5.Text = "Main"
        Me.TabPage5.UseVisualStyleBackColor = True
        '
        'TableLayoutPanel1
        '
        Me.TableLayoutPanel1.ColumnCount = 3
        Me.TableLayoutPanel1.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50.0!))
        Me.TableLayoutPanel1.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle())
        Me.TableLayoutPanel1.ColumnStyles.Add(New System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50.00001!))
        Me.TableLayoutPanel1.Controls.Add(Me.lblSerie, 0, 0)
        Me.TableLayoutPanel1.Controls.Add(Me.FlowLayoutPanel1, 1, 0)
        Me.TableLayoutPanel1.Controls.Add(Me.lblPedido, 2, 0)
        Me.TableLayoutPanel1.Dock = System.Windows.Forms.DockStyle.Bottom
        Me.TableLayoutPanel1.Location = New System.Drawing.Point(3, 866)
        Me.TableLayoutPanel1.Name = "TableLayoutPanel1"
        Me.TableLayoutPanel1.RowCount = 1
        Me.TableLayoutPanel1.RowStyles.Add(New System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 100.0!))
        Me.TableLayoutPanel1.Size = New System.Drawing.Size(1878, 61)
        Me.TableLayoutPanel1.TabIndex = 1613
        '
        'FlowLayoutPanel1
        '
        Me.FlowLayoutPanel1.Controls.Add(Me.Button2)
        Me.FlowLayoutPanel1.Controls.Add(Me.btnLogin)
        Me.FlowLayoutPanel1.Controls.Add(Me.btnReimprimir)
        Me.FlowLayoutPanel1.Controls.Add(Me.btnManual)
        Me.FlowLayoutPanel1.Controls.Add(Me.Button1)
        Me.FlowLayoutPanel1.Location = New System.Drawing.Point(573, 3)
        Me.FlowLayoutPanel1.Name = "FlowLayoutPanel1"
        Me.FlowLayoutPanel1.Size = New System.Drawing.Size(731, 55)
        Me.FlowLayoutPanel1.TabIndex = 0
        '
        'Button2
        '
        Me.Button2.Font = New System.Drawing.Font("Tahoma", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.Button2.Location = New System.Drawing.Point(3, 3)
        Me.Button2.Name = "Button2"
        Me.Button2.Size = New System.Drawing.Size(140, 47)
        Me.Button2.TabIndex = 1610
        Me.Button2.Text = "Imagenes"
        Me.Button2.UseVisualStyleBackColor = True
        '
        'btnLogin
        '
        Me.btnLogin.Font = New System.Drawing.Font("Tahoma", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.btnLogin.Location = New System.Drawing.Point(149, 3)
        Me.btnLogin.Name = "btnLogin"
        Me.btnLogin.Size = New System.Drawing.Size(140, 47)
        Me.btnLogin.TabIndex = 6
        Me.btnLogin.Text = "Login"
        Me.btnLogin.UseVisualStyleBackColor = True
        '
        'btnReimprimir
        '
        Me.btnReimprimir.Font = New System.Drawing.Font("Tahoma", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.btnReimprimir.Location = New System.Drawing.Point(295, 3)
        Me.btnReimprimir.Name = "btnReimprimir"
        Me.btnReimprimir.Size = New System.Drawing.Size(140, 47)
        Me.btnReimprimir.TabIndex = 5
        Me.btnReimprimir.Text = "Reimprimir"
        Me.btnReimprimir.UseVisualStyleBackColor = True
        '
        'btnManual
        '
        Me.btnManual.Font = New System.Drawing.Font("Tahoma", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.btnManual.Location = New System.Drawing.Point(441, 3)
        Me.btnManual.Name = "btnManual"
        Me.btnManual.Size = New System.Drawing.Size(140, 47)
        Me.btnManual.TabIndex = 4
        Me.btnManual.Text = "Manual"
        Me.btnManual.UseVisualStyleBackColor = True
        '
        'Button1
        '
        Me.Button1.Font = New System.Drawing.Font("Tahoma", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.Button1.Location = New System.Drawing.Point(587, 3)
        Me.Button1.Name = "Button1"
        Me.Button1.Size = New System.Drawing.Size(140, 47)
        Me.Button1.TabIndex = 3
        Me.Button1.Text = "Liberar"
        Me.Button1.UseVisualStyleBackColor = True
        '
        'TabPage1
        '
        Me.TabPage1.Controls.Add(Me.btnPruebas)
        Me.TabPage1.Controls.Add(Me.dtgUsers)
        Me.TabPage1.Controls.Add(Me.lblPCName)
        Me.TabPage1.Controls.Add(Me.CheckBoxMapicsServer)
        Me.TabPage1.Controls.Add(Me.dtgItems)
        Me.TabPage1.Controls.Add(Me.dtgMapics)
        Me.TabPage1.Controls.Add(Me.dtg_error)
        Me.TabPage1.Controls.Add(Me.dtgHistorico)
        Me.TabPage1.Location = New System.Drawing.Point(4, 22)
        Me.TabPage1.Name = "TabPage1"
        Me.TabPage1.Padding = New System.Windows.Forms.Padding(3)
        Me.TabPage1.Size = New System.Drawing.Size(1884, 930)
        Me.TabPage1.TabIndex = 0
        Me.TabPage1.Text = "Config"
        Me.TabPage1.UseVisualStyleBackColor = True
        '
        'btnPruebas
        '
        Me.btnPruebas.Location = New System.Drawing.Point(1803, 901)
        Me.btnPruebas.Name = "btnPruebas"
        Me.btnPruebas.Size = New System.Drawing.Size(75, 23)
        Me.btnPruebas.TabIndex = 6
        Me.btnPruebas.Text = "Pruebas"
        Me.btnPruebas.UseVisualStyleBackColor = True
        '
        'CheckBoxMapicsServer
        '
        Me.CheckBoxMapicsServer.AutoSize = True
        Me.CheckBoxMapicsServer.Location = New System.Drawing.Point(1654, 905)
        Me.CheckBoxMapicsServer.Name = "CheckBoxMapicsServer"
        Me.CheckBoxMapicsServer.Size = New System.Drawing.Size(143, 17)
        Me.CheckBoxMapicsServer.TabIndex = 0
        Me.CheckBoxMapicsServer.Text = "Servidor Mapics Pruebas"
        Me.CheckBoxMapicsServer.UseVisualStyleBackColor = True
        '
        'dtgMapics
        '
        Me.dtgMapics.AllowUserToAddRows = False
        Me.dtgMapics.AllowUserToDeleteRows = False
        Me.dtgMapics.AllowUserToResizeColumns = False
        Me.dtgMapics.AllowUserToResizeRows = False
        Me.dtgMapics.AutoSizeColumnsMode = System.Windows.Forms.DataGridViewAutoSizeColumnsMode.AllCells
        Me.dtgMapics.AutoSizeRowsMode = System.Windows.Forms.DataGridViewAutoSizeRowsMode.AllCells
        Me.dtgMapics.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize
        Me.dtgMapics.Location = New System.Drawing.Point(614, 6)
        Me.dtgMapics.Name = "dtgMapics"
        Me.dtgMapics.Size = New System.Drawing.Size(600, 400)
        Me.dtgMapics.TabIndex = 1
        '
        'dtg_error
        '
        Me.dtg_error.AllowUserToAddRows = False
        Me.dtg_error.AllowUserToDeleteRows = False
        Me.dtg_error.BackgroundColor = System.Drawing.Color.White
        Me.dtg_error.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize
        Me.dtg_error.Columns.AddRange(New System.Windows.Forms.DataGridViewColumn() {Me.ColumnFechaHora, Me.ColumnTitulo, Me.ColumnDescripcion})
        Me.dtg_error.Location = New System.Drawing.Point(1223, 6)
        Me.dtg_error.MultiSelect = False
        Me.dtg_error.Name = "dtg_error"
        Me.dtg_error.SelectionMode = System.Windows.Forms.DataGridViewSelectionMode.FullRowSelect
        Me.dtg_error.Size = New System.Drawing.Size(665, 400)
        Me.dtg_error.TabIndex = 3
        '
        'ColumnFechaHora
        '
        Me.ColumnFechaHora.AutoSizeMode = System.Windows.Forms.DataGridViewAutoSizeColumnMode.AllCells
        Me.ColumnFechaHora.HeaderText = "Fecha/Hora"
        Me.ColumnFechaHora.Name = "ColumnFechaHora"
        Me.ColumnFechaHora.ReadOnly = True
        Me.ColumnFechaHora.Width = 88
        '
        'ColumnTitulo
        '
        Me.ColumnTitulo.AutoSizeMode = System.Windows.Forms.DataGridViewAutoSizeColumnMode.AllCells
        Me.ColumnTitulo.HeaderText = "Titulo"
        Me.ColumnTitulo.Name = "ColumnTitulo"
        Me.ColumnTitulo.ReadOnly = True
        Me.ColumnTitulo.Width = 58
        '
        'ColumnDescripcion
        '
        Me.ColumnDescripcion.AutoSizeMode = System.Windows.Forms.DataGridViewAutoSizeColumnMode.Fill
        Me.ColumnDescripcion.HeaderText = "Descripcion"
        Me.ColumnDescripcion.Name = "ColumnDescripcion"
        Me.ColumnDescripcion.ReadOnly = True
        '
        'dtgHistorico
        '
        Me.dtgHistorico.AllowUserToAddRows = False
        Me.dtgHistorico.AllowUserToDeleteRows = False
        DataGridViewCellStyle1.BackColor = System.Drawing.Color.White
        DataGridViewCellStyle1.ForeColor = System.Drawing.Color.Black
        DataGridViewCellStyle1.SelectionBackColor = System.Drawing.Color.White
        DataGridViewCellStyle1.SelectionForeColor = System.Drawing.Color.Black
        Me.dtgHistorico.AlternatingRowsDefaultCellStyle = DataGridViewCellStyle1
        Me.dtgHistorico.AutoSizeColumnsMode = System.Windows.Forms.DataGridViewAutoSizeColumnsMode.AllCells
        Me.dtgHistorico.AutoSizeRowsMode = System.Windows.Forms.DataGridViewAutoSizeRowsMode.AllCells
        Me.dtgHistorico.BackgroundColor = System.Drawing.Color.White
        DataGridViewCellStyle2.Alignment = System.Windows.Forms.DataGridViewContentAlignment.MiddleLeft
        DataGridViewCellStyle2.BackColor = System.Drawing.SystemColors.Control
        DataGridViewCellStyle2.Font = New System.Drawing.Font("Tahoma", 8.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        DataGridViewCellStyle2.ForeColor = System.Drawing.SystemColors.WindowText
        DataGridViewCellStyle2.SelectionBackColor = System.Drawing.Color.White
        DataGridViewCellStyle2.SelectionForeColor = System.Drawing.Color.Black
        DataGridViewCellStyle2.WrapMode = System.Windows.Forms.DataGridViewTriState.[True]
        Me.dtgHistorico.ColumnHeadersDefaultCellStyle = DataGridViewCellStyle2
        Me.dtgHistorico.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize
        DataGridViewCellStyle3.Alignment = System.Windows.Forms.DataGridViewContentAlignment.MiddleLeft
        DataGridViewCellStyle3.BackColor = System.Drawing.SystemColors.Window
        DataGridViewCellStyle3.Font = New System.Drawing.Font("Tahoma", 8.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        DataGridViewCellStyle3.ForeColor = System.Drawing.SystemColors.ControlText
        DataGridViewCellStyle3.SelectionBackColor = System.Drawing.Color.White
        DataGridViewCellStyle3.SelectionForeColor = System.Drawing.Color.Black
        DataGridViewCellStyle3.WrapMode = System.Windows.Forms.DataGridViewTriState.[False]
        Me.dtgHistorico.DefaultCellStyle = DataGridViewCellStyle3
        Me.dtgHistorico.EditMode = System.Windows.Forms.DataGridViewEditMode.EditProgrammatically
        Me.dtgHistorico.Enabled = False
        Me.dtgHistorico.GridColor = System.Drawing.Color.WhiteSmoke
        Me.dtgHistorico.Location = New System.Drawing.Point(8, 6)
        Me.dtgHistorico.Name = "dtgHistorico"
        DataGridViewCellStyle4.Alignment = System.Windows.Forms.DataGridViewContentAlignment.MiddleLeft
        DataGridViewCellStyle4.BackColor = System.Drawing.Color.White
        DataGridViewCellStyle4.Font = New System.Drawing.Font("Tahoma", 8.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        DataGridViewCellStyle4.ForeColor = System.Drawing.Color.Black
        DataGridViewCellStyle4.SelectionBackColor = System.Drawing.Color.White
        DataGridViewCellStyle4.SelectionForeColor = System.Drawing.Color.Black
        DataGridViewCellStyle4.WrapMode = System.Windows.Forms.DataGridViewTriState.[True]
        Me.dtgHistorico.RowHeadersDefaultCellStyle = DataGridViewCellStyle4
        Me.dtgHistorico.RowHeadersWidth = 8
        Me.dtgHistorico.SelectionMode = System.Windows.Forms.DataGridViewSelectionMode.FullRowSelect
        Me.dtgHistorico.Size = New System.Drawing.Size(600, 916)
        Me.dtgHistorico.TabIndex = 0
        '
        'txtRawScanner2
        '
        Me.txtRawScanner2.Anchor = CType((System.Windows.Forms.AnchorStyles.Bottom Or System.Windows.Forms.AnchorStyles.Right), System.Windows.Forms.AnchorStyles)
        Me.txtRawScanner2.CharacterCasing = System.Windows.Forms.CharacterCasing.Upper
        Me.txtRawScanner2.Font = New System.Drawing.Font("Tahoma", 6.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.txtRawScanner2.Location = New System.Drawing.Point(1621, 974)
        Me.txtRawScanner2.Name = "txtRawScanner2"
        Me.txtRawScanner2.ReadOnly = True
        Me.txtRawScanner2.Size = New System.Drawing.Size(261, 17)
        Me.txtRawScanner2.TabIndex = 1609
        Me.txtRawScanner2.TabStop = False
        Me.txtRawScanner2.TextAlign = System.Windows.Forms.HorizontalAlignment.Center
        '
        'Form1
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.White
        Me.ClientSize = New System.Drawing.Size(1904, 993)
        Me.Controls.Add(Me.TabControl1)
        Me.Controls.Add(Me.txtRawScanner)
        Me.Controls.Add(Me.txtRawScanner2)
        Me.Controls.Add(Me.StatusStrip1)
        Me.Font = New System.Drawing.Font("Tahoma", 8.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.Name = "Form1"
        Me.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen
        Me.Text = "Hussmann Packout"
        Me.WindowState = System.Windows.Forms.FormWindowState.Maximized
        Me.StatusStrip1.ResumeLayout(False)
        Me.StatusStrip1.PerformLayout()
        CType(Me.dtgUsers, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.dtgItems, System.ComponentModel.ISupportInitialize).EndInit()
        Me.TabControl1.ResumeLayout(False)
        Me.TabPage5.ResumeLayout(False)
        Me.TableLayoutPanel1.ResumeLayout(False)
        Me.FlowLayoutPanel1.ResumeLayout(False)
        Me.TabPage1.ResumeLayout(False)
        Me.TabPage1.PerformLayout()
        CType(Me.dtgMapics, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.dtg_error, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.dtgHistorico, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub

    Friend WithEvents StatusStrip1 As StatusStrip
    Friend WithEvents lblFecha As ToolStripStatusLabel
    Friend WithEvents lblHora As ToolStripStatusLabel
    Friend WithEvents Timer1 As Timer
    Friend WithEvents txtRawScanner As TextBox
    Friend WithEvents lblPedido As Label
    Friend WithEvents lblSerie As Label
    Friend WithEvents dtgMapics As DataGridView
    Friend WithEvents lblError As ToolStripStatusLabel
    Friend WithEvents dtgUsers As DataGridView
    Friend WithEvents Button1 As Button
    Friend WithEvents dtgItems As DataGridView
    Friend WithEvents ColumnSerie As DataGridViewTextBoxColumn
    Friend WithEvents itemsC As DataGridViewTextBoxColumn
    Friend WithEvents txtRawScanner2 As TextBox
    Friend WithEvents TabControl1 As TabControl
    Friend WithEvents TabPage1 As TabPage
    Friend WithEvents CheckBoxMapicsServer As CheckBox
    Friend WithEvents btnManual As Button
    Friend WithEvents btnReimprimir As Button
    Friend WithEvents dtg_error As DataGridView
    Friend WithEvents ColumnFechaHora As DataGridViewTextBoxColumn
    Friend WithEvents ColumnTitulo As DataGridViewTextBoxColumn
    Friend WithEvents ColumnDescripcion As DataGridViewTextBoxColumn
    Friend WithEvents lblPCName As Label
    Friend WithEvents btnPruebas As Button
    Friend WithEvents btnLogin As Button
    Friend WithEvents lblUserLogin As ToolStripStatusLabel
    Friend WithEvents TabPage5 As TabPage
    Friend WithEvents dtgHistorico As DataGridView
    Friend WithEvents flpImagenes As FlowLayoutPanel
    Friend WithEvents Button2 As Button
    Friend WithEvents TableLayoutPanel1 As TableLayoutPanel
    Friend WithEvents FlowLayoutPanel1 As FlowLayoutPanel
    Friend WithEvents lblSQL As ToolStripStatusLabel
    Friend WithEvents lblItems As ToolStripStatusLabel
    Friend WithEvents lblescanner As ToolStripStatusLabel
End Class
