local ls = require 'luasnip'

return {
	ls.parser.parse_snippet("expand", "--this is expanded")
}
