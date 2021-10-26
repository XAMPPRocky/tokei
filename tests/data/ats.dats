//! 42 lines 25 code 9 comments 8 blanks

(*************
    Reference:
    https://github.com/ats-lang/ats-lang.github.io/blob/master/DOCUMENT/ATS2TUTORIAL/CODE/chap_stream_vt.dats
**************)
#include "share/atspre_staload.hats"

/* Lazy-evaluated integer iterator */
fun from
(n: int): stream_vt(int) =
  $ldelay(stream_vt_cons(n, from(n + 1)))

// Lazy-evaluated prime finder
fun sieve
(ns: stream_vt(int))
: stream_vt(int) = $ldelay(
  let
    val ns_con = !ns
    val- @stream_vt_cons(n0, ns1) = ns_con
    val n0_val = n0
    val ns1_val = ns1

    val () =
      (ns1 := sieve(stream_vt_filter_cloptr<int>(ns1_val, lam x => x mod n0_val > 0)))
    
    prval () = fold@(ns_con)
  in
    ns_con
  end
  ,
  ~ns
)

// Test run for finding the 1000-th prime number
val thePrimes = sieve(from(2))
val p1000 = (steam_vt_drop_exn(thePrimes, 1000)).head()
val () = println!("p1000 = ", p1000)

implement main0 () = {}

(* End of file *)
