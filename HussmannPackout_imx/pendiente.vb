Imports System.ComponentModel
Public Class pendiente
    Private Sub Button1_Click(sender As Object, e As EventArgs) Handles Button1.Click
        Sqll.insrt("N/A", Form1.lblUserLogin.Text.ToUpper, TextBox1.Text & " items " & txtItemsPdte.Text)
        Form1.vali1 = False
        Form1.vali2 = False
        Form1.vali3 = False
        Form1.lblSerie.Text = ""
        Form1.lblPedido.Text = ""
        'Form1.lblRes.Text = ""
        'Form1.lblItemsInd.ForeColor = Color.Black
        'Form1.lblBatchInd.ForeColor = Color.Black
        'Form1.lblNoSerieInd.ForeColor = Color.Black
        For i = 0 To 99
            Form1.lbl_items(i).Text = ""
        Next
        Form1.txtRawScanner.Text = ""
        Form1.txtRawScanner.SelectAll()
        Form1.WindowState = FormWindowState.Maximized
        Form1.LoginUsr = False
        Me.Close()
    End Sub
    Private Sub txtBatch_TextChanged(sender As Object, e As EventArgs) Handles txtBatch.TextChanged
        Sqll.ConsultaUsrAdmin(txtBatch.Text.Replace("(", ""))
        If DataGridView1.Rows.Count > 0 Then
            Button1.Enabled = True
        Else
            Button1.Enabled = False
        End If
    End Sub
    Private Sub pendiente_Load(sender As Object, e As EventArgs) Handles MyBase.Load
        'If Form1.dtgItems.Rows.Count > 0 Then
        '    For i = 0 To Form1.dtgItems.Rows.Count - 1
        '        txtItemsPdte.Text = txtItemsPdte.Text & Form1.dtgItems.Item(0, i).Value.ToString & " - " & Form1.dtgItems.Item(1, i).Value.ToString & vbNewLine
        '    Next
        'End If
        Button1.Enabled = False
        txtBatch.SelectAll()
    End Sub
    Private Sub pendiente_Closing(sender As Object, e As CancelEventArgs) Handles Me.Closing
        Form1.WindowState = FormWindowState.Maximized
    End Sub
End Class