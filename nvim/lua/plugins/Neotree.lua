return {
  "nvim-neo-tree/neo-tree.nvim",
  enabled = true,
  opts = {
    source_selector = {
      winbar = true,
      statusline = true,
    },
    window = {
      width = 30,
      mappings = {
        S = "split_with_window_picker",
        s = "vsplit_with_window_picker",
      },
    },
  },
  dependencies = {
    { "s1n7ax/nvim-window-picker" },
  },
}
