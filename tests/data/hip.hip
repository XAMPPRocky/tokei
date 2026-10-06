// 28 lines 17 code 6 comments 5 blanks

#include <hip/hip_runtime.h>

/* Add two vectors.
   Block comments can span lines. */
__global__ void add(const int* a, const int* b, int* c) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    c[i] = a[i] + b[i]; // inline comment
}

int main() {
    // Comment markers inside strings are code.
    const char* markers = "// /* */";
    const char* escaped = "\" // still a string";
    const char slash = '/';
    const char quote = '\'';

    const char* raw = R"(A raw string with "quotes"
// not a comment
/* not a block comment */
)";

    /* A block comment with "quotes" and // markers. */
    hipLaunchKernelGGL(add, dim3(1), dim3(64), 0, 0, nullptr, nullptr, nullptr);
    return hipDeviceSynchronize();
}
// End of HIP example.
