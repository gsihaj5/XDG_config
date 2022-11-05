local ls = require 'luasnip'

-- snippet creator
local s = ls.snippet

-- creating format string with insert node
local fmt = require("luasnip.extras.fmt").fmt

-- insert node for {} in fmt
local i = ls.insert_node

-- repeat input from insert node
local rep = require('luasnip.extras').rep
local f = ls.function_node
return {
	s = s,
	i = i,
	f = f,
	fmt = fmt,
	rep = rep,
}
