vim.cmd('let g:airline_powerline_fonts = 1')

--nerd tree
vim.cmd('nnoremap <space>n :NERDTreeToggle<CR>')
require('plugins-config.telescope')
require('plugins-config.vim-closetag')
require('plugins-config.luasnip-conf')
require('plugins-config.vimspector')
require('plugins-config.nerdcommenter')

local autocmd = require('au')

autocmd.BufEnter = { '*', function()
		local filetype = vim.bo.filetype
		if filetype == 'blade' then
			vim.cmd("source $HOME/.config/nvim/lua/plugins-config/coc.vim")
		else
			vim.cmd("let b:coc_suggest_disable = 1")
			require('plugins-config.nvim-cmp')
			require('plugins-config.builtin-lsp')
		end
    end,
}


