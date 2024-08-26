# 17 lines 12 code 3 comments 2 blanks

# Below is a function
(defn a-fn
  "Docstring with a hash #"
  [a b]
  (+ 1 1))

(defn a-fn2
  #"Not a doc"
  "String"
  [a b] # a and b right?
  (let [multiline "I'm
  a multline
  # string
  "]
       (str multline a b)))
