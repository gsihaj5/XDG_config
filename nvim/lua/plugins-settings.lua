vim.cmd('let g:airline_powerline_fonts = 1')

--nerd tree
vim.cmd('nnoremap <space>n :NERDTreeToggle<CR>')
require('plugins-config.telescope')

local autocmd = require('au')

autocmd.BufEnter = { '*', function()
		local filetype = vim.bo.filetype
		if filetype == 'blade' then
			print("coc")
			vim.cmd("source $HOME/.config/nvim/lua/plugins-config/coc.vim")
		else
			print("builtin lsp")
			require('plugins-config.nvim-cmp')
			require('plugins-config.builtin-lsp')
		end
    end,
}


