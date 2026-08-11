Public Class FormReimprimir
    Private Sub FormReimprimir_Load(sender As Object, e As EventArgs) Handles MyBase.Load
        Me.Text = "Packout - " & Form1.ver
        ConsultaUltimosSeriesPackout(txtCajasEReimprimir)
    End Sub
    Private Sub btnCajasEReimprimir_Click(sender As Object, e As EventArgs) Handles btnCajasEReimprimir.Click
        btnCajasEReimprimir.Enabled = False
        reimprimir()
        Form1.LoginUsr = False
        btnCajasEReimprimir.Enabled = True
        Me.Close()
    End Sub
    Public Sub reimprimir()
        mapics.borrar(txtCajasEReimprimir.Text)
        mapics.insert(txtCajasEReimprimir.Text)
        Sqll.insrt("N/A", Form1.lblUserLogin.Text, "Serie reimpresa: " & txtCajasEReimprimir.Text)
        Sqll.Err("Reimpreción", "Numero de serie reimpreso: " & txtCajasEReimprimir.Text)
    End Sub
End Class