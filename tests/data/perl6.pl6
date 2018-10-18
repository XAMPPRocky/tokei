# 11 lines 5 code 4 comments 2 blanks

=begin pod
Defines a fun infix operator.
This was stolen from http://tpm2016.zoffix.com/#/14
=end pod
sub infix:<¯\(°_o)/¯> {
    ($^a, $^b).pick
}

say 'Coke' ¯\(°_o)/¯ 'Pepsi';
