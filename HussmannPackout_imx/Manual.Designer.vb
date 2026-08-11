<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class Manual
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
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(Manual))
        Me.txtManual = New System.Windows.Forms.TextBox()
        Me.btnAceptarManual = New System.Windows.Forms.Button()
        Me.SuspendLayout()
        '
        'txtManual
        '
        Me.txtManual.BackColor = System.Drawing.Color.White
        Me.txtManual.Font = New System.Drawing.Font("Tahoma", 48.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.txtManual.Location = New System.Drawing.Point(12, 12)
        Me.txtManual.Name = "txtManual"
        Me.txtManual.Size = New System.Drawing.Size(776, 85)
        Me.txtManual.TabIndex = 0
        Me.txtManual.TextAlign = System.Windows.Forms.HorizontalAlignment.Center
        '
        'btnAceptarManual
        '
        Me.btnAceptarManual.Location = New System.Drawing.Point(351, 103)
        Me.btnAceptarManual.Name = "btnAceptarManual"
        Me.btnAceptarManual.Size = New System.Drawing.Size(75, 23)
        Me.btnAceptarManual.TabIndex = 1
        Me.btnAceptarManual.Text = "&Aceptar"
        Me.btnAceptarManual.UseVisualStyleBackColor = True
        '
        'Manual
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.DodgerBlue
        Me.ClientSize = New System.Drawing.Size(800, 134)
        Me.Controls.Add(Me.btnAceptarManual)
        Me.Controls.Add(Me.txtManual)
        Me.Font = New System.Drawing.Font("Tahoma", 8.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.MaximumSize = New System.Drawing.Size(816, 173)
        Me.MinimizeBox = False
        Me.MinimumSize = New System.Drawing.Size(816, 173)
        Me.Name = "Manual"
        Me.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen
        Me.Text = "Manual"
        Me.TopMost = True
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub

    Friend WithEvents txtManual As TextBox
    Friend WithEvents btnAceptarManual As Button
End Class
