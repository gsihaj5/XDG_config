return {
  "Pocco81/auto-save.nvim",
  lazy = false,
  opts = {
    debounce_delay = 500,
    execution_message = {
      message = function()
        return ""
      end,
    },
    condition = function(buf)
      if vim.bo[buf].filetype == "harpoon" then
        return false
      end
      return true
    end,
  },
  keys = {
    { "<leader>uv", "<cmd>ASToggle<CR>", desc = "Toggle autosave" },
  },
}
