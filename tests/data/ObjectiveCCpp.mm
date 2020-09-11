//21 lines, 8 code, 9 comments, 4 blanks

// First program example
 
#import <Foundation/Foundation.h>

/*
A multi-line comment can be inserted
by using a slash
and then an asterisk.
The comment can be as long as you want,
but make sure you close it
with an asterisk, then a slash. */
 
int main (int argc, const char * argv[])
{
        NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];
        NSLog (@"Hello, World!");
        [pool drain];
        return 0;
}