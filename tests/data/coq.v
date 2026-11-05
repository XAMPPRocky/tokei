(* 16 lines 8 code 5 comments 3 blanks *)
Definition backslash := "\".
(* A backslash does not escape the closing quote. *)

Definition backslashes := "\\\".
(* Neither does an odd run of backslashes. *)

Definition quote := """".
Definition quoted := "a ""(* still a string *)"" \".
(* Doubled quotes and backslashes leave this a comment. *)

Definition multiline := "first ""
(* still a string *)
"" last\".
(* The multiline string has ended. *)
Definition empty := "".
