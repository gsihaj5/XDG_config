require('plugins')
--plugin settings meant to settings mappings and properties for plugins
require('plugins-settings')

--lsp configuration
require('lsp.denols')
require('lsp.psalm')
require('lsp.sumneko_lua')
require('lsp.intelephense')
require('lsp.tailwindcss')

local autocmd = require('au')

require('plugins-config.builtin-lsp')
autocmd.BufEnter = {
    '*.blade.php',
    function()
		vim.cmd("source $HOME/.config/nvim/lua/plugins-config/coc.vim")
    end,
}


--this is basic settings
require('settings')
require('mappings')
