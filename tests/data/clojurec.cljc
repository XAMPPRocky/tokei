; 19 lines 13 code 2 comments 4 blank

(ns clojure)

; Below is a function
(defn a-fn
  "Docstring with a column ;"
  [a b]
  (+ 1 1))

(defn a-fn2
  ;"Not a doc"
  "Doc doc again"
  [a b] ; a and b right?
  (let [multiline "I'm
  a multline
  ; string
  "]
       (str multline a b)))
