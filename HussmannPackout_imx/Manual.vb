Imports System.ComponentModel
Public Class Manual
    Private Sub Manual_Load(sender As Object, e As EventArgs) Handles MyBase.Load
        txtManual.SelectAll()
    End Sub
    Private Sub txtManual_Leave(sender As Object, e As EventArgs) Handles txtManual.Leave
        txtManual.BackColor = Color.Red
        txtManual.Focus()
        txtManual.Select()
    End Sub
    Private Sub txtManual_LostFocus(sender As Object, e As EventArgs) Handles txtManual.LostFocus
        txtManual.BackColor = Color.Red
        txtManual.Focus()
        txtManual.Select()
    End Sub
    Private Sub txtManual_GotFocus(sender As Object, e As EventArgs) Handles txtManual.GotFocus
        txtManual.BackColor = Color.DodgerBlue
        txtManual.ForeColor = Color.White
    End Sub
    Public Sub AceptSerialNumber()
        Form1.Obs = "Manual"
        Form1.lblescanner.Text = txtManual.Text
        Form1.txtRawScanner.Text = txtManual.Text.Replace(" ", "")
        Form1.ValidateSN()
    End Sub
    Private Sub btnAceptarManual_Click(sender As Object, e As EventArgs) Handles btnAceptarManual.Click
        btnAceptarManual.Enabled = False
        AceptSerialNumber()
        Sqll.insrt("N/A", Form1.lblUserLogin.Text.ToUpper, "Manual: " & txtManual.Text)
        Form1.LoginUsr = False
        Me.Close()
        btnAceptarManual.Enabled = True
    End Sub
    Private Sub Manual_Closing(sender As Object, e As CancelEventArgs) Handles Me.Closing
        Form1.WindowState = FormWindowState.Maximized
    End Sub
    Private Sub Form1_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Enter Then
            enterV()
        End If
    End Sub
    Private Sub txtManual_TextChanged(sender As Object, e As EventArgs) Handles txtManual.TextChanged
        AceptSerialNumber()
    End Sub
    Private Sub txtManual_KeyDown(ByVal sender As Object, ByVal e As System.Windows.Forms.KeyEventArgs) Handles txtManual.KeyDown
        If e.KeyCode = Keys.Enter Then
            enterV()
        End If
    End Sub
    Public Sub enterV()
        If txtManual.Text.Length > 0 Then
            Form1.ValidateSN()
            Me.Close()
        End If
    End Sub
End Class