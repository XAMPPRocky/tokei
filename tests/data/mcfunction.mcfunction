# 10 lines 4 code 2 comments 4 blanks

gamerule commandBlockOutput false

execute as @a run say Hello world!
# Single line comment

summon cow ~ ~ ~ {CustomName:'{"text":"Definitely not a cow"}'}

execute if score xWires my_objective matches 1..10 run say my_objective is between 1 and 10 (inclusive)