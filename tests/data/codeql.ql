//! 40 lines 17 code 15 comments 8 blanks

/** 
 * @name fu 
 * @description bar 
 *
 * Rerum similique consequatur non dolor sit. Autem doloribus sed in sint
 * ratione sit voluptates at. Nihil ut fugiat ab ut aliquid consequatur sunt
 * ullam. Adipisci voluptatem hic dicta.
 */

// asdf

import cpp
private import test.foo.bar.baz

/**
 * Another comment.
 */
class C extends Expr {
    C () {
        // single comment
        not this.test() and
        not this.what()
    }

    private predicate what() {
        /* TODO */
        this.isAbstract()
    }

    predicate test() { this = "what" }
}

from Function f
where 
    f.getName() = "function" and /* inline comment */
    f.getArgument(0).asExpr() instanceof FooBar
select f, "function"

