// 11 lines 6 code 3 comments 2 blanks
// The 'foo.aleo' program.
program foo.aleo;

/**********************************************************************************************************************/

function hello:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
