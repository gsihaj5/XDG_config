-- Options are automatically loaded before lazy.nvim startup
-- Default options that are always set: https://github.com/LazyVim/LazyVim/blob/main/lua/lazyvim/config/options.lua
-- Add any additional options here
--
local opt = vim.opt
opt.winbl = 10
opt.colorcolumn = "72"
opt.scrolloff = 18
opt.foldmethod = "marker"
opt.foldexpr = "nvim_treesitter#foldexpr()"
opt.shiftwidth = 4

vim.g.autoformat = false
