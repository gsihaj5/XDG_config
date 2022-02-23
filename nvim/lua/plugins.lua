return require('packer').startup(function()
	-- packer to update itself
	use 'wbthomason/packer.nvim'

  -- config files for built in lsp client
  use 'neovim/nvim-lspconfig'

  -- lsp installer
  use 'williamboman/nvim-lsp-installer'

  -- color theme
  use 'morhetz/gruvbox'

  --status bar
  use 'vim-airline/vim-airline'

  use 'powerline/powerline'


end)
