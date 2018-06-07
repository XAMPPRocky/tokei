' 34 lines 29 code 3 comments 2 blanks
VERSION 5.00
Begin VB.Form Form1 
   Caption         =   "Form1"
   ClientHeight    =   3015
   ClientLeft      =   120
   ClientTop       =   465
   ClientWidth     =   4560
   LinkTopic       =   "Form1"
   ScaleHeight     =   3015
   ScaleWidth      =   4560
   StartUpPosition =   3  'Windows Default
   Begin VB.CommandButton btnTest 
      Caption         =   "Test"
      Height          =   495
      Left            =   720
      TabIndex        =   0
      Top             =   840
      Width           =   1095
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
'This is a comment
Private Sub btnTest_Click()

    'This is another comment
    Dim test As String
    test = "TESTING"

End Sub
