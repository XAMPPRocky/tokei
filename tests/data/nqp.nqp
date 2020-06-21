# 24 lines 14 code 8 comments 2 blanks
=begin
Regex methods and functions
=end

=begin item match
Match C<$text> against C<$regex>.  If the C<$global> flag is
given, then return an array of all non-overlapping matches.
=end item

sub match ($text, $regex, :$global?) {
    my $match := $text ~~ $regex;
    if $global {
        my @matches;
        while $match {
            nqp::push(@matches, $match);
            $match := $match.parse($text, :rule($regex), :c($match.to));
        }
        @matches;
    }
    else {
        $match;
    }
}
