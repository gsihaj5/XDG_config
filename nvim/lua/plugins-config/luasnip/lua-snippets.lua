local su = require('plugins-config.luasnip.luasnip-util')

return {
	su.s("req", su.fmt(
		"local {} = {}",
		{ su.i(1), su.rep(1) }
	))
}
