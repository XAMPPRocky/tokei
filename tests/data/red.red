Red [] ; 20 lines 10 code 6 comments 4 blanks

x: 1 ; line comment 1
x: 3   ;-- line comment 2
x: 3		;@@ line comment 3

comment ['this
	'is 'multiline
	'comment]
comment {and this
	as well}

function add100 [x [integer!]] [
	"; this should not count as comment "
	{ comment [and neither
	 this] }
]

comment   {unfortunately, tokei does not allow regexp in comment prefix}
; so probably previous block comment is not parsed properly
