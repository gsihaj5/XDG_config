local ls = require("luasnip")
local s = ls.s
local i = ls.insert_node
local f = ls.function_node
local fmt = require("luasnip.extras.fmt").fmt
local rep = require("luasnip.extras").rep

ls.add_snippets("typescriptreact", {
  s(
    "rfc",
    fmt(
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
    )
  ),
})

ls.add_snippets("typescriptreact", {
  s(
    "nt",
    fmt(
      [[
        type T{} = {{
        }}
        export default T{}
        ]],
      {
        i(1, "<typeName>"),
        rep(1),
      }
    )
  ),
})

ls.add_snippets("typescriptreact", {
  s(
    "ne",
    fmt(
      [[
        export enum {} {{
        }}
        ]],
      {
        i(1, "<enumName>"),
      }
    )
  ),
})

-- create state and capitalize word after set State
ls.add_snippets("typescriptreact", {
  s(
    "us",
    fmt(
      [[
        const [{}, set{}] = useState({})
        ]],
      {
        i(1, "<state>"),
        f(function(args)
          local word = args[1][1]
          return word:sub(1, 1):upper() .. word:sub(2)
        end, { 1 }),
        i(2, "<initial>"),
      }
    )
  ),
})

-- console log with variable name
ls.add_snippets("typescriptreact", {
  s(
    "cl",
    fmt(
      [[
        console.log("{}", {})
        ]],
      {
        i(1, "<variable>"),
        rep(1),
      }
    )
  ),
})
