# 49 lines 37 code 6 comments 6 blanks

=begin pod

=begin DESCRIPTION

=head1 Test file for Tokei

=end DESCRIPTION

=begin code :lang<raku>

say 'Hello World';

=end code

=end pod

#| Fibonacci with multiple dispatch
multi sub fib (0 --> 0) {}
multi sub fib (1 --> 1) {}
multi sub fib (\n where * > 1) {
    fib(n - 1) + fib(n - 2)
}

#|{
Role shape
for printing area of different shapes
}
role Shape {
    method area { ... }

    method print_area {
        say "Area of {self.^name} is {self.area}.";
    }
}

class Rectangle does Shape {
    has $.width is required;   #= Width of rectangle
    has $.height is required;  #= Height of rectangle

    method area {
        #`(
        area of rectangle:
        width times height
         )
        $!width × $!height
    }
}
