⍝ 10 lines 3 code 3 comments 4 blanks


256=2*8

⍝ Comment
⊃¨ ' '(≠⊆⊢) 'A Programming Language'

⍝ A magic square of length ⊢
MS ← (⍳-∘⌈÷∘2)(⊣⊖⌽),⍨⍴∘⍳×⍨
