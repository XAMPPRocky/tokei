// 13 lines 7 code 3 comments 3 blanks

use fmt;

// Main function
export fn main() void = {
	let msg = "// a comment";
	let ch = '\''; // not a rune
	fmt::println(msg)!;
	fmt::printfln("a rune {}", ch)!;
};

// end
