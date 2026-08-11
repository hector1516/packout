<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class FormImagenes
    Inherits System.Windows.Forms.Form

    'Form reemplaza a Dispose para limpiar la lista de componentes.
    <System.Diagnostics.DebuggerNonUserCode()> _
    Protected Overrides Sub Dispose(ByVal disposing As Boolean)
        Try
            If disposing AndAlso components IsNot Nothing Then
                components.Dispose()
            End If
        Finally
            MyBase.Dispose(disposing)
        End Try
    End Sub

    'Requerido por el Diseñador de Windows Forms
    Private components As System.ComponentModel.IContainer

    'NOTA: el Diseñador de Windows Forms necesita el siguiente procedimiento
    'Se puede modificar usando el Diseñador de Windows Forms.  
    'No lo modifique con el editor de código.
    <System.Diagnostics.DebuggerStepThrough()> _
    Private Sub InitializeComponent()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(FormImagenes))
        Me.ofdImagen = New System.Windows.Forms.OpenFileDialog()
        Me.txtItem = New System.Windows.Forms.TextBox()
        Me.txtRutaImagen = New System.Windows.Forms.TextBox()
        Me.btnBuscar = New System.Windows.Forms.Button()
        Me.pbVistaPrevia = New System.Windows.Forms.PictureBox()
        Me.btnGuardar = New System.Windows.Forms.Button()
        Me.btnEliminar = New System.Windows.Forms.Button()
        Me.btnLimpiar = New System.Windows.Forms.Button()
        Me.dtgImagenesGuardadas = New System.Windows.Forms.DataGridView()
        Me.Label1 = New System.Windows.Forms.Label()
        Me.Label2 = New System.Windows.Forms.Label()
        CType(Me.pbVistaPrevia, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.dtgImagenesGuardadas, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'ofdImagen
        '
        Me.ofdImagen.FileName = "OpenFileDialog1"
        '
        'txtItem
        '
        Me.txtItem.Location = New System.Drawing.Point(15, 25)
        Me.txtItem.Name = "txtItem"
        Me.txtItem.Size = New System.Drawing.Size(318, 20)
        Me.txtItem.TabIndex = 0
        '
        'txtRutaImagen
        '
        Me.txtRutaImagen.Location = New System.Drawing.Point(15, 64)
        Me.txtRutaImagen.Name = "txtRutaImagen"
        Me.txtRutaImagen.Size = New System.Drawing.Size(318, 20)
        Me.txtRutaImagen.TabIndex = 1
        '
        'btnBuscar
        '
        Me.btnBuscar.Location = New System.Drawing.Point(15, 90)
        Me.btnBuscar.Name = "btnBuscar"
        Me.btnBuscar.Size = New System.Drawing.Size(75, 23)
        Me.btnBuscar.TabIndex = 2
        Me.btnBuscar.Text = "Buscar"
        Me.btnBuscar.UseVisualStyleBackColor = True
        '
        'pbVistaPrevia
        '
        Me.pbVistaPrevia.Location = New System.Drawing.Point(15, 119)
        Me.pbVistaPrevia.Name = "pbVistaPrevia"
        Me.pbVistaPrevia.Size = New System.Drawing.Size(318, 318)
        Me.pbVistaPrevia.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom
        Me.pbVistaPrevia.TabIndex = 3
        Me.pbVistaPrevia.TabStop = False
        '
        'btnGuardar
        '
        Me.btnGuardar.Location = New System.Drawing.Point(96, 90)
        Me.btnGuardar.Name = "btnGuardar"
        Me.btnGuardar.Size = New System.Drawing.Size(75, 23)
        Me.btnGuardar.TabIndex = 4
        Me.btnGuardar.Text = "Guardar"
        Me.btnGuardar.UseVisualStyleBackColor = True
        '
        'btnEliminar
        '
        Me.btnEliminar.Location = New System.Drawing.Point(177, 90)
        Me.btnEliminar.Name = "btnEliminar"
        Me.btnEliminar.Size = New System.Drawing.Size(75, 23)
        Me.btnEliminar.TabIndex = 5
        Me.btnEliminar.Text = "Eliminar"
        Me.btnEliminar.UseVisualStyleBackColor = True
        '
        'btnLimpiar
        '
        Me.btnLimpiar.Location = New System.Drawing.Point(258, 90)
        Me.btnLimpiar.Name = "btnLimpiar"
        Me.btnLimpiar.Size = New System.Drawing.Size(75, 23)
        Me.btnLimpiar.TabIndex = 6
        Me.btnLimpiar.Text = "Limpiar"
        Me.btnLimpiar.UseVisualStyleBackColor = True
        '
        'dtgImagenesGuardadas
        '
        Me.dtgImagenesGuardadas.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize
        Me.dtgImagenesGuardadas.Location = New System.Drawing.Point(339, 25)
        Me.dtgImagenesGuardadas.Name = "dtgImagenesGuardadas"
        Me.dtgImagenesGuardadas.Size = New System.Drawing.Size(318, 412)
        Me.dtgImagenesGuardadas.TabIndex = 7
        '
        'Label1
        '
        Me.Label1.AutoSize = True
        Me.Label1.Location = New System.Drawing.Point(12, 9)
        Me.Label1.Name = "Label1"
        Me.Label1.Size = New System.Drawing.Size(27, 13)
        Me.Label1.TabIndex = 8
        Me.Label1.Text = "Item"
        '
        'Label2
        '
        Me.Label2.AutoSize = True
        Me.Label2.Location = New System.Drawing.Point(12, 48)
        Me.Label2.Name = "Label2"
        Me.Label2.Size = New System.Drawing.Size(68, 13)
        Me.Label2.TabIndex = 9
        Me.Label2.Text = "Ruta Imagen"
        '
        'FormImagenes
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.ClientSize = New System.Drawing.Size(665, 450)
        Me.Controls.Add(Me.Label2)
        Me.Controls.Add(Me.Label1)
        Me.Controls.Add(Me.dtgImagenesGuardadas)
        Me.Controls.Add(Me.btnLimpiar)
        Me.Controls.Add(Me.btnEliminar)
        Me.Controls.Add(Me.btnGuardar)
        Me.Controls.Add(Me.pbVistaPrevia)
        Me.Controls.Add(Me.btnBuscar)
        Me.Controls.Add(Me.txtRutaImagen)
        Me.Controls.Add(Me.txtItem)
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.MaximumSize = New System.Drawing.Size(681, 489)
        Me.MinimizeBox = False
        Me.MinimumSize = New System.Drawing.Size(681, 489)
        Me.Name = "FormImagenes"
        Me.ShowInTaskbar = False
        Me.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen
        Me.Text = "Items"
        Me.TopMost = True
        CType(Me.pbVistaPrevia, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.dtgImagenesGuardadas, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub

    Friend WithEvents ofdImagen As OpenFileDialog
    Friend WithEvents txtItem As TextBox
    Friend WithEvents txtRutaImagen As TextBox
    Friend WithEvents btnBuscar As Button
    Friend WithEvents pbVistaPrevia As PictureBox
    Friend WithEvents btnGuardar As Button
    Friend WithEvents btnEliminar As Button
    Friend WithEvents btnLimpiar As Button
    Friend WithEvents dtgImagenesGuardadas As DataGridView
    Friend WithEvents Label1 As Label
    Friend WithEvents Label2 As Label
End Class
