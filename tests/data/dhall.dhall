-- 16 lines 9 code 5 comments 2 blanks
{- A comment within the interior of a multi-line literal counts as part of the
   literal
-}

''
-- Hello
{- world -}
''
{ some = "thing"

, keys = ["can"
, "have",
-- wait for it
"lists"]
}
