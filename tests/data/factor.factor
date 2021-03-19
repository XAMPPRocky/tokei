! 14 lines, 5 code, 6 comments, 3 blanks

/* we can use some dependencies */
USING: math multiline sequences ;

! this is a vocabulary
IN: my-vocab

! this comment describes this function
: add ( x y -- z )
    "Hello World  !\
    " length /*
        Add the three numbers.
    */ + + ;
