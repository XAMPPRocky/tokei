# 16 lines, 9 code, 5 blanks, 2 comments
echo "This is a
multiline string
# with a hash
in it."

echo 'This is a single-quoted string.'

# This is a comment.

use re

edit:after-readline = [
  [line]{ print "\e]2;"$line"\a" > /dev/tty }
]

