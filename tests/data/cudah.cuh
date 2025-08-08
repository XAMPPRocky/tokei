/* 15 lines 7 code 6 comments 2 blanks */

// A template class
template<typename T>
struct Array {
    T *d;
    size_t count;

    /**
     * @brief Constructor for Array.
     */
    __host__ __device__ Array() : d(nullptr), count(0) {
        // A line comment
    }
};
