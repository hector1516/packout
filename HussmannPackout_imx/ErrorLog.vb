Imports System.Runtime.CompilerServices

Module ErrorLog
    Public Sub logg(ByVal msge As String, ByVal errorr As String)
        FormError.Show()
        FormError.TextBox1.Text = DateTime.Now.ToString & vbNewLine & msge & vbNewLine & errorr
    End Sub

End Module
