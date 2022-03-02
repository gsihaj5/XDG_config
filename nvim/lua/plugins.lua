return require('packer').startup(function()
	-- packer to update itself
	use 'wbthomason/packer.nvim'

  -- config files for built in lsp client
  use 'neovim/nvim-lspconfig'

  -- lsp installer
  -- use :LspInstall
  use 'williamboman/nvim-lsp-installer'

  -- color theme
  use 'morhetz/gruvbox'

  --status bar
  use 'vim-airline/vim-airline'
  use 'powerline/powerline'

  --tree explorer
  use 'preservim/nerdtree'


  -- auto completion
  use 'hrsh7th/cmp-nvim-lsp'
  use 'hrsh7th/cmp-buffer'
  use 'hrsh7th/cmp-path'
  use 'hrsh7th/cmp-cmdline'
  use 'hrsh7th/nvim-cmp'

  use 'L3MON4D3/LuaSnip'
  use 'saadparwaiz1/cmp_luasnip'

  --coc
  use {'neoclide/coc.nvim', run = 'yarn install'}
  --dependency for vim blade
  use 'jwalton512/vim-blade'

  --telescope for file finder
  use {
  	'nvim-telescope/telescope.nvim',
  	requires = { {'nvim-lua/plenary.nvim'} }
  }
  
  --js syntax highlight
  use 'mxw/vim-jsx'
  use 'pangloss/vim-javascript'

  --tag completion
  use 'alvan/vim-closetag'

end)
