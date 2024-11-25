# 29 lines 17 code 6 comments 7 blanks

##
# foo function doing stuff
#
proc foo {bar {baz 5} \
    {not_used "hi mom"}} {
    puts $bar ;# print out bar
    # sum bar + baz 
    set foo_res [expr $bar + $baz]
    return $foo_res
}

proc #weird {} {
    return -code 5
}


puts "hello world ####"

puts #Alsovalid# ;# comment here!
puts {;#and this}
puts {really anything -$#;/ /\ \ -_'@?` "\".$?"!'][¨@ is allowed in curly braces}

puts "3 == [foo 1 2] and 6 == [foo 2]"

set function "#weird"
set code [catch {[$function]}]
puts "$code == 5"
