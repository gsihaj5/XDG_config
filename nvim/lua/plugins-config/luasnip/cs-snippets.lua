local su = require('plugins-config.luasnip.luasnip-util')

return {
	su.s("class",
		su.t("public Class "),
		su.i(1),
		su.t("{}")
	--su.f(function(args, snip)
	--local env = snip.env
	--local split_filename = su.splitString(env.TM_FILENAME, ".")
	--return args[1][1] .. split_filename[1]
	--end),
	--su.t("{}")
	),
	su.ls.parser.parse_snippet("wl", "Console.WriteLine($1);"),
	su.ls.parser.parse_snippet("ofn", "public $1 $2() \n{\n$0\n}")
}
