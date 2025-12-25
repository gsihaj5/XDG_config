return {
  "MeanderingProgrammer/render-markdown.nvim",
  config = function()
    require("render-markdown").setup({
      -- render_modes = { "n", "c", "t" },
      render_modes = true,
      anti_conceal = { enabled = true },
    })
  end,
}
