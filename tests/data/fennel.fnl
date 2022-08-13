;; 18 lines 8 code 5 comments 5 blanks

; this is a ; single comment
;;;; this is also a single comment ;;;;;;

                  ; "this is a comment too!"

(local variable "I ;am a ;variable!")

; (print "\"I am commented out!\"")
(print "\"Hello world!\"") ; this is an ; end of line comment
(print "This is not a comment: ;")
(print "This is a
  multiline string")

(fn somefn [x]
  (print "I am some function.")
  (print "My parameter is " (string.format "\"%s\"" x)))
