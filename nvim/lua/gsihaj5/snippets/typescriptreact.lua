local ls = require('luasnip')
local s = ls.s
local fmt = require("luasnip.extras.fmt").fmt
local i = ls.insert_node
local rep = require("luasnip.extras").rep

ls.add_snippets('typescriptreact', {
    s('rfc', fmt(
        [[
        interface I{}Props {{
        }}
        const {}: React.FC<I{}Props> = () => {{
            return <>{}</>
        }}
        ]],
        {
            i(1, "default"),
            rep(1),
            rep(1),
            rep(1),
        }
    )),
})
