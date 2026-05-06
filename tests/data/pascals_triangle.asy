//! 39 lines 21 code 9 comments 9 blanks

/*
 * A visualization of Pascal's triangle that shows it's semblance with Sierpinski's triangle.
 */

// The `level` parameter corresponds to the desired iteration of Sierpinski's recursive construction.
int level = 5;

// The `height` parameter determines the "physical" height of the diagram.
real height = 8cm;

// Auxiliary constants
int n = 2 ^ level;
int[][] values = new int[n][n];

unitsize(height / n); // Ensure that the output size is constant

// Directional vectors
pair o = (0, 0);
pair u = unit((1, -sqrt(3)));
pair v = unit((1, sqrt(3)));

// The actual implementation
for (int i = 0; i < n; ++i) {
  for (int j = 0; i + j < n; ++j) {
    if (i == 0 || j == 0) {
      values[i][j] = 1;
    } else {
      values[i][j] = values[i - 1][j] + values[i][j - 1];
    }

    pair p = i * u - j * v;

    if (values[i][j] % 2 == 1 /* We may want to invert this condition if another coloring is desired */) {
      fill(shift(i * u - j * v) * scale(1 / 2) * unitcircle);
    }
  }
}
