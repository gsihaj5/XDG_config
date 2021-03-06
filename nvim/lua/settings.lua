vim.o.ruler = true
vim.o.showcmd = true
vim.o.showmode = true

vim.o.cmdheight = 2

-- Sidebar
vim.o.number = true -- line number on the left
vim.o.numberwidth = 3 -- always reserve 3 spaces for line number
vim.o.signcolumn = 'yes' -- keep 1 column for coc.vim  check
vim.o.modelines = 0
vim.o.relativenumber = true --relative number based on cursor pos

-- White characters
vim.o.autoindent = true
vim.o.smartindent = true
vim.o.tabstop = 4 -- 1 tab = 2 spaces
vim.o.shiftwidth = 4 -- indentation rule
vim.o.formatoptions = 'qnj1' -- q  - comment formatting; n - numbered lists; j - remove comment when joining lines; 1 - don't break after one-letter word
vim.o.expandtab = false -- expand tab to spaces

vim.cmd(":syntax on")

--theme settings
vim.cmd("colorscheme gruvbox")
vim.cmd("set t_Co=256")
vim.cmd("set cc=80")
vim.cmd("highlight ColorColumn ctermbg=lightgrey guibg=NONE")
vim.cmd("highlight Normal ctermbg=NONE guibg=NONE")
vim.cmd("let g:gruvbox_transparent_bg = 0")

vim.cmd("set completeopt=menu,menuone")

vim.cmd("set encoding=utf8")

