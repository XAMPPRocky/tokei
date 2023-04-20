"16 lines 9 code 5 comment 2 blank"

exampleWithNumber: x
    | y |
    "this is a comment"
    true & false not & (nil isNil) ifFalse: [self halt].
    y := self size + super size.
    #($a #a 'a' 1 1.0)
        do: [ :each |

            "This is 
            a multiline 
            comment"
            Transcript show: (each class name);
                       show: ' '].
    ^x < y
