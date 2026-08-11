<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class Pruebas
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
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(Pruebas))
        Me.btnInsert = New System.Windows.Forms.Button()
        Me.txtSerieInsert = New System.Windows.Forms.TextBox()
        Me.GroupBoxLog = New System.Windows.Forms.GroupBox()
        Me.txtLog = New System.Windows.Forms.TextBox()
        Me.btnDel = New System.Windows.Forms.Button()
        Me.Label1 = New System.Windows.Forms.Label()
        Me.Button1 = New System.Windows.Forms.Button()
        Me.GroupBoxLog.SuspendLayout()
        Me.SuspendLayout()
        '
        'btnInsert
        '
        Me.btnInsert.Location = New System.Drawing.Point(302, 25)
        Me.btnInsert.Name = "btnInsert"
        Me.btnInsert.Size = New System.Drawing.Size(114, 23)
        Me.btnInsert.TabIndex = 2
        Me.btnInsert.Text = "Insert ESTPACK01"
        Me.btnInsert.UseVisualStyleBackColor = True
        '
        'txtSerieInsert
        '
        Me.txtSerieInsert.Location = New System.Drawing.Point(12, 25)
        Me.txtSerieInsert.Name = "txtSerieInsert"
        Me.txtSerieInsert.Size = New System.Drawing.Size(199, 21)
        Me.txtSerieInsert.TabIndex = 1
        '
        'GroupBoxLog
        '
        Me.GroupBoxLog.Controls.Add(Me.txtLog)
        Me.GroupBoxLog.Location = New System.Drawing.Point(12, 52)
        Me.GroupBoxLog.Name = "GroupBoxLog"
        Me.GroupBoxLog.Size = New System.Drawing.Size(760, 269)
        Me.GroupBoxLog.TabIndex = 1
        Me.GroupBoxLog.TabStop = False
        Me.GroupBoxLog.Text = "Log"
        '
        'txtLog
        '
        Me.txtLog.BackColor = System.Drawing.Color.AntiqueWhite
        Me.txtLog.Location = New System.Drawing.Point(6, 20)
        Me.txtLog.Multiline = True
        Me.txtLog.Name = "txtLog"
        Me.txtLog.ReadOnly = True
        Me.txtLog.Size = New System.Drawing.Size(748, 243)
        Me.txtLog.TabIndex = 0
        '
        'btnDel
        '
        Me.btnDel.Location = New System.Drawing.Point(542, 25)
        Me.btnDel.Name = "btnDel"
        Me.btnDel.Size = New System.Drawing.Size(75, 23)
        Me.btnDel.TabIndex = 2
        Me.btnDel.Text = "Borrar"
        Me.btnDel.UseVisualStyleBackColor = True
        '
        'Label1
        '
        Me.Label1.AutoSize = True
        Me.Label1.Location = New System.Drawing.Point(9, 9)
        Me.Label1.Name = "Label1"
        Me.Label1.Size = New System.Drawing.Size(86, 13)
        Me.Label1.TabIndex = 3
        Me.Label1.Text = "Numero de Serie"
        '
        'Button1
        '
        Me.Button1.Location = New System.Drawing.Point(422, 25)
        Me.Button1.Name = "Button1"
        Me.Button1.Size = New System.Drawing.Size(114, 23)
        Me.Button1.TabIndex = 4
        Me.Button1.Text = "Insert ESTPACK02"
        Me.Button1.UseVisualStyleBackColor = True
        '
        'Pruebas
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.White
        'Me.BackgroundImage = Global.HussmannPackout.My.Resources.Resources.engrane_marca_agua
        Me.ClientSize = New System.Drawing.Size(784, 328)
        Me.Controls.Add(Me.Button1)
        Me.Controls.Add(Me.Label1)
        Me.Controls.Add(Me.btnDel)
        Me.Controls.Add(Me.btnInsert)
        Me.Controls.Add(Me.txtSerieInsert)
        Me.Controls.Add(Me.GroupBoxLog)
        Me.Font = New System.Drawing.Font("Tahoma", 8.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.MaximumSize = New System.Drawing.Size(800, 367)
        Me.MinimizeBox = False
        Me.Name = "Pruebas"
        Me.Text = "Packout - Pruebas"
        Me.GroupBoxLog.ResumeLayout(False)
        Me.GroupBoxLog.PerformLayout()
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub
    Friend WithEvents GroupBoxLog As GroupBox
    Friend WithEvents txtLog As TextBox
    Friend WithEvents btnInsert As Button
    Friend WithEvents txtSerieInsert As TextBox
    Friend WithEvents btnDel As Button
    Friend WithEvents Label1 As Label
    Friend WithEvents Button1 As Button
End Class
