;;; 26 lines 14 code 4 comments 8 blanks

(import (srfi srfi-1)) ; for reduce

;; Constant
(define %pi 3.14159265358979323846)

#| This is a block comment |#
(define (degrees->radians deg)
  (* deg (/ %pi 180)))

;; Function
(define (sq x) (* x x))

(define (sum xs)
  "Sum list of elements."
  (reduce + 0 xs)) ; comment

(define (sum-upto n)
  (/ (* n (1+ n)) 2))

(define (test-sums n)
  (= (sum-upto n)
     (sum (iota (1+ n)))))

(test-sums 100)
