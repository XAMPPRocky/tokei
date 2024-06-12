NB. 27 lines 16 code 4 comments 7 blanks

foo=: 5 NB. uncounted comment

NB. single line comment
bar=: 'A string with ''quotes'''

ed=: 0 : 0
explicit defined string
)

ed2=: 3 :0
 +: y NB. uncounted comment inside explicit definition
)

dd1=: {{)n
explicit defined string using direct definitions
}}

Note 
multi line comment with the `Note` keyword 
cannot be identified at the moment
)

NB. another single line comment
NB. consecutive comments
dd2=: {{ x * y }} + 8 
