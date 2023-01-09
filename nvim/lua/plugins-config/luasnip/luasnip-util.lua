local ls = require 'luasnip'

-- snippet creator
local ps = ls.parser.parse_snippet
local s = ls.snippet

-- creating format string with insert node
local fmt = require("luasnip.extras.fmt").fmt
local fmta = require("luasnip.extras.fmt").fmta

-- insert node for {} in fmt
local i = ls.insert_node
local t = ls.text_node

-- repeat input from insert node
local rep = require('luasnip.extras').rep
local f = ls.function_node
local splitString = function(inputstr, sep)
	if sep == nil then
		sep = "%s"
	end
	local t = {}
	for str in string.gmatch(inputstr, "([^" .. sep .. "]+)") do
		table.insert(t, str)
	end
	return t
end
return {
	ls = ls,
	ps = ps,
	s = s,
	i = i,
	f = f,
	fmt = fmt,
	fmta = fmta,
	t = t,
	rep = rep,
	splitString = splitString
}
