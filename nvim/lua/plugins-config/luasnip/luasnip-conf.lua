local ls = require "luasnip"

ls.config.set_config {
	history = true,
	updateevents = "TextChanged, TextChangedI",
	enable_autosnippets = true,
}

local path = "plugins-config.luasnip."

local filetypes = {
	"lua",
	"cs",
}
local all_snippets = require(path .. 'all-snippets')
ls.add_snippets("all", all_snippets)

for index, filetype in pairs(filetypes) do
	local snippets_path = require(path .. filetype .. '-snippets')
	ls.add_snippets(filetype, snippets_path)
end
