return {
  -- change markdown previewer options
  {
    "iamcco/markdown-preview.nvim",
    opts = {
            
      -- set default theme (dark or light)
      -- By default the theme is defined according to the preferences of the system
      vim.g.mkdp_theme == "light",
      -- other options can be added here
    },
  },
}
