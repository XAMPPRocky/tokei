# 36 lines 18 code 10 comments 8 blanks
module [square]
# this is a comment
# this is another comment

a1 = 1
a2 = 3.14159 # pi

expect
    # simple check
    a1 == 1

expect
    a2 |> Num.toStr == "3.14159"

## Compute the square
square = \x ->
    s = x * x

    # the line above is blank
    s

expect square 3 == 9

## """
## this is not a multiline string,
## it's a doc comment
## """
multilineString =
    """
    # this line is not a comment, it's actually code

    The line above is not blank, it's actually code
    """

expect multilineString |> Str.toUtf8 |> List.first == Ok '#'
