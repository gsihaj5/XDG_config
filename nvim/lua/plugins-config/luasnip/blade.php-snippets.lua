local ls = require 'luasnip'

return {
	ls.parser.parse_snippet("blade", "--this is expanded")
}
