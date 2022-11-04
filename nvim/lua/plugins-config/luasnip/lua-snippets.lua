local ls = require 'luasnip'

-- snippet creator
local s = ls.s

-- creating format string with insert node
local fmt = require("luasnip.extras.fmt").fmt

-- insert node for {} in fmt
local i = ls.insert_node

-- repeat input from insert node
local rep = require('luasnip.extras').rep
local ai = require("luasnip.nodes.absolute_indexer")

return {
	s("req", fmt("local {} = {}", { i(1), rep(ai[1]) }))
}
