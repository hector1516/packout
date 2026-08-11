<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class FormReimprimir
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
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(FormReimprimir))
        Me.GroupBox3 = New System.Windows.Forms.GroupBox()
        Me.txtCajasEReimprimir = New System.Windows.Forms.ComboBox()
        Me.btnCajasEReimprimir = New System.Windows.Forms.Button()
        Me.Label5 = New System.Windows.Forms.Label()
        Me.GroupBox3.SuspendLayout()
        Me.SuspendLayout()
        '
        'GroupBox3
        '
        Me.GroupBox3.BackColor = System.Drawing.Color.Transparent
        Me.GroupBox3.Controls.Add(Me.txtCajasEReimprimir)
        Me.GroupBox3.Controls.Add(Me.btnCajasEReimprimir)
        Me.GroupBox3.Controls.Add(Me.Label5)
        Me.GroupBox3.Font = New System.Drawing.Font("Tahoma", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.GroupBox3.Location = New System.Drawing.Point(12, 12)
        Me.GroupBox3.Name = "GroupBox3"
        Me.GroupBox3.Size = New System.Drawing.Size(496, 103)
        Me.GroupBox3.TabIndex = 10
        Me.GroupBox3.TabStop = False
        Me.GroupBox3.Text = "Reimprimir Etiqueta"
        '
        'txtCajasEReimprimir
        '
        Me.txtCajasEReimprimir.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList
        Me.txtCajasEReimprimir.FormattingEnabled = True
        Me.txtCajasEReimprimir.Location = New System.Drawing.Point(11, 57)
        Me.txtCajasEReimprimir.Name = "txtCajasEReimprimir"
        Me.txtCajasEReimprimir.Size = New System.Drawing.Size(325, 33)
        Me.txtCajasEReimprimir.TabIndex = 3
        '
        'btnCajasEReimprimir
        '
        Me.btnCajasEReimprimir.Location = New System.Drawing.Point(342, 57)
        Me.btnCajasEReimprimir.Name = "btnCajasEReimprimir"
        Me.btnCajasEReimprimir.Size = New System.Drawing.Size(144, 33)
        Me.btnCajasEReimprimir.TabIndex = 2
        Me.btnCajasEReimprimir.Text = "Reimprimir"
        Me.btnCajasEReimprimir.UseVisualStyleBackColor = True
        '
        'Label5
        '
        Me.Label5.AutoSize = True
        Me.Label5.Location = New System.Drawing.Point(6, 29)
        Me.Label5.Name = "Label5"
        Me.Label5.Size = New System.Drawing.Size(390, 25)
        Me.Label5.TabIndex = 1
        Me.Label5.Text = "Ultimos 20 Numeros de Serie Aprobados"
        '
        'FormReimprimir
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.White
        Me.ClientSize = New System.Drawing.Size(524, 124)
        Me.Controls.Add(Me.GroupBox3)
        Me.Font = New System.Drawing.Font("Tahoma", 8.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.MinimizeBox = False
        Me.Name = "FormReimprimir"
        Me.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent
        Me.Text = "FormReimprimir"
        Me.TopMost = True
        Me.GroupBox3.ResumeLayout(False)
        Me.GroupBox3.PerformLayout()
        Me.ResumeLayout(False)

    End Sub

    Friend WithEvents GroupBox3 As GroupBox
    Friend WithEvents txtCajasEReimprimir As ComboBox
    Friend WithEvents btnCajasEReimprimir As Button
    Friend WithEvents Label5 As Label
End Class
