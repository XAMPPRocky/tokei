// 33 lines 23 code 5 comments 5 blanks

/* AL is the language of Microsoft Dynamics 365 Business Central.
   This block comment spans three lines and should be counted
   as three comment lines. */

namespace MyCompany.Sales;

using Microsoft.Foundation;

table 50100 "My Table"
{
    fields
    {
        field(1; Name; Text[50])
        {
            Caption = 'Name';
        }
    }

    /* a single line block comment */
    procedure Compute(Value: Decimal): Decimal
    var
        Total: Decimal;
        Note: Text;
    begin
        Total := Value * 2;  // double it
        Note := 'It''s at https://example.com';
        Note := 'not /* a comment */ either';
        Note := '// not a comment either';
        exit(Total);
    end;
}
