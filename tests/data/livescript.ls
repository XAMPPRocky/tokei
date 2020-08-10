# 28 lines, 10 code, 12 comments, 6 blanks

/*
 * /* Nested comment
 * #  single line comment
 * */

/*

add = (a, b) ->
  return a + b
*/

hello = ->
  console.log 'hello, world!'

"hello!" |> capitalize |> console.log

# Easy listing of implicit objects
table1 =
  * id: 1
    name: 'george'
  * id: 2
    name: 'mike'  # comment
  * id: 3
    name: 'donald'

# Comment
