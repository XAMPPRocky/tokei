// 34 lines 23 code 7 comments 4 blanks
pragma circom 2.0.8;

/*
 * Sum an array of non-zero values.
 */
function sum(values, size) {
  var sum = 0;
  for (var i = 0; i < size; i++) {
    assert(values[i] != 0);
    sum += values[i];
  }
  log("sum = ", sum);
  return sum;
}

/*
 * Ensure x is a solution to x^5 - 2x^4 + 5x - 4 = 0.
 */
template Polynomial() {
    signal input x;
    signal x2;
    signal x4;
    signal x5;
    signal output y;

    x2 <== x * x;
    x4 <== x2 * x2;
    x5 <== x4 * x;
    y <== x5 - 2 * x4 + 5 * x - 4;      // y = x^5 - 2 * x^4 + 5x - 4.
    y === 0;                            // Ensure that y = 0.
}

component main = Polynomial();
