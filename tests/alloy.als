// 18 lines 10 code 3 comments 5 blanks

sig Node {
    edge: set Node
}

------------------------------------------------------------------------

pred self_loop[n: Node] {
  n in n.edge
}

pred all_self_loop {
  all n: Node | self_loop[n]
}

/* Comments started by /* don't nest */
run all_self_loop
