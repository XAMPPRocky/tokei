;;; 40 lines 15 code 14 comments 11 blanks
#lang racket ; defines the language we are using

;;; Comments

;; Single line comments start with a semicolon

#| Block comments
   can span multiple lines and...
    #|
        they can be nested!
    |#
|#

;; S-expression comments discard the following expression
;; since this is syntax-aware, tokei counts this as code
#; (this expression is discarded)

;; Constant
(define %pi 3.14159265358979323846)

#| This is a block comment |#
(define (degrees->radians deg)
  (* deg (/ %pi 180)))

;; Function
(define (sq x) (* x x))

(define (sum xs)
  "Sum list of elements."
  (foldl + 0 xs)) ; comment

(define (sum-upto n)
  (/ (* n (+ 1 n)) 2))

(define (test-sums n)
  (= (sum-upto n)
     (sum (range (+ 1 n)))))

(test-sums 100)
