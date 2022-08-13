#!/bin/ksh
# 17 lines, 11 code, 4 comments, 2 blanks

# first comment
files="/etc/passwd /etc/group /etc/hosts"
for f in $files; do
    if [ ! -f $f ]
    then
        echo "$f file missing!"
    fi
done

# second comment
for f in $(ls /tmp/*)
do
    print "Full file path in /tmp dir : $f"
done
