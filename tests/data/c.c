// 50 lines 33 code 8 comments 9 blanks

/* /* we can't nest block comments in c, but we can start one */
int main(void) {
	char *start = "/*";

	int x = 1;
	x += 2; // end of line comment */
}

void foo() {
	char *esc = "\"/*escaped quotes in a string and block comment*/\"";
	func1();
	func2();
	char *next_line =
		"*/ /*string on new line\
		continued to another line\
		bar();\
		*/";

	char *next_line2 = "line1\
		// not a real comment\
		line3*/";

	/* Block comment
	// line comment in a block comment
	end block comment*/

	char *late_start = // "
		"wow\
		that's pretty neat";

	char *late_start2 = /* " */
		"*/ still just a string"; // but this is a line comment
}

void foobar() {
	int a = 4; // /*
    int b = 5;
    int c = 6; // */
}

/*\
 / comment
\*/
struct Point {
    int x;
    int y;
    int z;
};
