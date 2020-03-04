# 15 lines 7 code 5 comments 3 blanks
#
# This is a comment line. We don't have multi-comment lines
#

macro define offsetof(_type, _memb)       ((long)(&((_type *)0)->_memb))

break foo
continue

# Let's have something print when a breakpoint is hit.
commands 2
  p i
  continue
end
