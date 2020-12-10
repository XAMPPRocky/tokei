-- 91 lines 48 code 34 comments 9 blanks

--[[
This is a test file for tokei parsing. It contains
x lines
y comments
z blanks
]]

local template = [=[

local function unpack(%s)
  --[[
    This function implements a specialized unpack operation
    which exhibits better jit behavior than the built in unpack
    by specializing on number of arguments
  ]]
  return %s
end

return unpack
]=]

local unpack_memo = {} -- a table which stores the generated functions
local function gen_unpack(i, j)
  if unpack_memo[i] and unpack_memo[i][j] then
    return unpack_memo[i][j]
  end
  local vals = {}
  for k = i, j do
    table.insert(vals, "arg["..tostring(k).."]")
  end
  local src = template:format("arg", table.concat(vals, ", "))
  local f = assert(loadstring(src))
  unpack_memo[i] = unpack_memo[i] or {}
  unpack_memo[i][j] = f
  return f
end

--[[
  This function implements unpack as specified in the stdlib in a way which is more amenable to JIT compilation
]]
local function unpack(list, i, j)
  if not j then j = #list end
  if not i then i = 1 end
  -- due to tracing JIT behavior, the tracing abort on closure construction or parsing code
  -- cannot cause compilation to fail, because the hotloop detection will only start a trace
  -- after the function generation is already memoized. The code traced through this function
  -- can be fully inlined and will have ~5 guards with a small number of instructions, thus
  -- having very low performance cost
  return gen_unpack(i, j)(list)
end

--[==[
Sample code using these:

local tab = {{1, 2}, {3, 4}}
--[[
  fold and map are standard functional list functions
  compose is function composition
  add and mul are function forms of the math operations
--]]
local sum_of_prods = fold(add, 0, map(compose(unpack, mul), t))

  [==[]===]]]]=]--this is to test the "closing" behavior'


]==]

--[[
  A translation of the famous fast reciprocal square root function in the quake engine
]]
local terra Q_rsqrt(x: float): float
  var i: int32
  var x2: float, y: float

  x2, y = x * 0.5f, x
  i = @([&int32]([&opaque](&x))) -- evil floating point bit level hacking
  i = 0x5f3759df - ( i >> 1 ) -- what the fuck?
  y = @([&float]([&opaque](&i)))
  y = y * (1.5f - (x * y * y)) -- 1st iteration
  -- y = y * (1.5f - (x * y * y)) -- 2nd iteration, this can be removed

  return y
end

return {
  gen_unpack = gen_unpack,
  unpack = unpack,
  Q_rsqrt = Q_rsqrt
}
