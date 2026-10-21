// 12 lines 9 code 2 comments 1 blanks
int f(const char *s) {
  if (s[0] == '"') {
    return 1;
  }
  if (s[0] == '\'') {
    return 2;
  }

  // quote handling done
  return 0;
}
