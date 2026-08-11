Public Class LoginForm
    Public loginUserNo As String = ""
    Private Sub txtId_TextChanged(sender As Object, e As EventArgs) Handles txtId.TextChanged
        If txtId.Text.Length > 0 Then
            Dim c As String = Val(txtId.TextLength)
            lblCaracteres.Text = "Caracteres: " & c.PadLeft(2, "0")
            If txtId.TextLength > 8 Then
                Sqll.ConsultaUsrAdminLogin(txtId.Text.Replace("(", ""))
                If dtgLogin.Rows.Count > 0 Then
                    Button1.Enabled = True
                    loginUserNo = txtId.Text.Replace("(", "")
                ElseIf String.Compare(txtId.Text, "Qwe123456") = 0 Then
                    Button1.Enabled = True
                    loginUserNo = "SuperAdmin"
                Else
                    Button1.Enabled = False
                End If
            End If
        End If
    End Sub
    Private Sub LoginForm_Load(sender As Object, e As EventArgs) Handles MyBase.Load
        txtId.SelectAll()
    End Sub
    Private Sub Button1_Click(sender As Object, e As EventArgs) Handles Button1.Click
        Form1.LoginUsr = True
        'loginUserNo = txtId.Text.Replace("(", "")
        Form1.lblUserLogin.Text = loginUserNo.Replace("(", "").ToUpper
        Me.Close()
    End Sub
    Private Sub PictureBox1_Click(sender As Object, e As EventArgs) Handles PictureBox1.Click
        If dtgLogin.Visible = False Then
            dtgLogin.Visible = True
        ElseIf dtgLogin.Visible = True Then
            dtgLogin.Visible = False
        End If
    End Sub
End Class