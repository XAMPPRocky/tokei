// 29 lines 17 code 7 comments 5 blanks
import "core:fmt"

/*
 * Calculates the next number in the Collatz sequence
 *
 * If `x` is divisible by two, the result is `x` divided by two
 * If `x` is not divisible by two, the result is `x` multiplied by three plus one
 */
collatz :: inline proc(x: int) -> int {
	if x & 1 == 0 do return x >> 1;
	else do return x * 3 + 1;
}

steps :: proc(x: int) -> int {
	count := 0;

	y := x;
	for y != 1 {
		y = collatz(y);
		count += 1;
	}

	return count;
}

main :: proc() {
	fmt.println(steps(42)); // 8
}