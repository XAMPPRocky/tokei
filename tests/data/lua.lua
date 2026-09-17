-- 48 lines 12 code 26 comments 10 blanks

-- Standard comment

-- Standard block comment
--[[
print("Hello world")
--]]

-- The following print statement isn't a commented
---[[
print("Hello world")
--]]

-- Another block comment
--[=[
We can include "]]" inside this comment
--]=]

--[==[
We can include "]=]" inside this comment
--]==]


local s = [[--[[ and -- don't start comments inside multiline strings.
A multiline string.
It can't contain double square brackets. Instead one must write: [=[]=].
]]

local s = [=[
Also a multiline string.
This one can contain [[]].
Alternatively, one may add one more equal sign between the square brackets: [==[]==]
]=]

-- Here only the first print statement is executed
---[[
print("Hello world 1")
--[=[]]
print("Hello world 2")
--]=]

-- Here only the second print statement is executed
--[[
print("Hello world 1")
--[=[]]
print("Hello world 2")
--]=]
