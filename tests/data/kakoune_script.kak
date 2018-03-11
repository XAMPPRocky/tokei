# 13 lines, 8 code, 2 comments, 3 blanks
hook global BufCreate (.*/)?(kakrc|.*.kak) %{
  set-option buffer filetype kak
}

echo "This is a string".
echo 'This is a
multiline string
# with a hash
in it.'

# This is a comment.

