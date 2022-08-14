/* 7 lines 4 code 2 comments 1 blanks */

// add vector
__host__ void add(const int* a, const int* b, int* c) {
    int i = threadIdx.x;
    c[i] = a[i] + b[i];
}
