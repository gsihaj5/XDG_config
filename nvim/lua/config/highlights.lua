print("HI")
local hl = vim.api.nvim_set_hl

hl(0, "@keyword.dbml", { link = "Constant" }) -- orange
hl(0, "@type.dbml", { link = "Type" })
hl(0, "@property.dbml", { link = "Identifier" })
hl(0, "@attribute.dbml", { link = "PreProc" })
hl(0, "@variable.dbml", { link = "Identifier" })
