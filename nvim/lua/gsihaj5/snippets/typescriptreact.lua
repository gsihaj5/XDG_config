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
        export default {};
        ]],
        {
            i(1, "default"),
            rep(1),
            rep(1),
            rep(1),
            rep(1),
        }
    )),
})

ls.add_snippets('typescriptreact', {
    s('nt', fmt(
        [[
        type T{} = {{
        }}
        export default T{}
        ]],
        {
            i(1, "<typeName>"),
            rep(1),
        }
    )),
})

ls.add_snippets('typescriptreact', {
    s('ne', fmt(
        [[
        export enum {} {{
        }}
        ]],
        {
            i(1, "<enumName>"),
        }
    )),
})
