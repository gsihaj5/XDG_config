return {
  "hrsh7th/nvim-cmp",
  dependencies = {
    "hrsh7th/cmp-emoji",
  },
  enabled = false,
  opts = function(_, opts)
    local cmp = require("cmp")

    opts.mapping = {
      ["<CR>"] = cmp.mapping.confirm({ select = true }), -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
    }
  end,
}
