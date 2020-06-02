// 8 lines 2 code 4 comments 2 blanks
/suduko solver

/ initial state
x:.:'"200370009009200007001004002050000800008000900006000040900100500800007600400089001"

/ breadth search all solutions (p:row col box for each position)
*(,x)(,/{@[x;y;:;]'&~in[!10]x*|/p[;y]=p,:3/:_(p:9\:!81)%3}')/&~x
