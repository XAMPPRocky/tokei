/* 16 lines 8 code 5 comments 3 blanks */
%module mymodule

/*
 * Wrapper-includes
 */
%{
#include "myheader.h" //dummy header
%}

// Now list ANSI C/C++ declarations
int foo;
int bar(int x);

%rename(my_print) print;
extern void print(const char *);
