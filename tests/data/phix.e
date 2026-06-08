/* 40 lines 25 code 8 comments 7 blanks */

-- copied from cpp, not necessarily idiomatic Euphoria code

include std/sequence.e

-- bubble_sort_function
public function bubble_sort(sequence a)
    integer t = 0
    integer j = length(a)
    integer s = 1
    while s > 0 do
        s = 0
        integer i = 2
        while i <= j do
            if a[i] < a[i - 1] then
                t = a[i]
                a[i] = a[i - 1]
                a[i - 1] = t
                s = 1
            end if
            i += 1
        end while
        j -= 1
    end while
    return a
end function

sequence a = {4, 65, 2, -31, 0, 99, 2, 83, 782, 1}

-- Single line comment
? {"Before:", a}

a = bubble_sort(a)

/* multi
 * line
 * comment
 */
? {"After:", a, equal(a, {-31,0,1,2,2,4,65,83,99,782})}