#!/bin/bash
# 43 lines 27 code 6 comments 10 blanks

: '
  This is
  a multine
  comment
'

str="hello
world
# this is not a comment
"
echo "${str}"

cat <<-EOF
# this is also not a comment
EOF

# But this is a comment
## So is this

function simple_hello() {
  echo '#hello' \
    | sed 's/#/!/' \
    | xargs
}

another_hello() {
  printf '%s\n' 'hello'
}

simple_hello
another_hello

var=1
echo "$((${var} + 1))" # Increment var # by one

for i in $(seq 1 10); do
  echo "${#i}"
done

# end of file
