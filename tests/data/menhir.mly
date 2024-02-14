// 47 lines 31 code 7 comments 9 blanks

(* Example from the menhir development with instrumented comments.
 * (* Note: nested C style comments are not allowed. *)
 * https://gitlab.inria.fr/fpottier/menhir/-/tree/master/demos/calc-alias *)

%token<int> INT  "42"
%token PLUS       "+"
%token MINUS      "-"
%token TIMES      "*"
%token DIV        "/"
%token LPAREN     "("
%token RPAREN     ")"
%token EOL

(* Token aliases can be used throughout the rest of the grammar. E.g.,
   they can be used in precedence declarations: *)

%left "+" "-"       /* lowest " precedence */
%left "*" "/"       /* medium precedence */
%nonassoc UMINUS    // highest "precedence"

%start <int> main

%%

main:
| e = expr EOL
    { e }

(* Token aliases can also be used inside rules: *)

expr:
| i = "42"
    { i }
| "(" e = expr ")"
    { e }
| e1 = expr "+" e2 = expr
    { e1 + e2 }
| e1 = expr "-" e2 = expr
    { e1 - e2 }
| e1 = expr "*" e2 = expr
    { e1 * e2 }
| e1 = expr "/" e2 = expr
    { e1 / e2 }
| "-" e = expr %prec UMINUS
    { - e }
