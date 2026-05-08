-- 20 lines 11 code 5 comments 4 blanks

-- Single-line comment
--[[
multiline comment with "quotes" and `backticks`
]]

local _single = 'single quoted'
local _double = "double quoted"
local _name = "Luau"

local _longString = [[
line one
line two
]]

local _interpolated = `hello {_name}`
local _multiline = `welcome {
    _name
}!`
