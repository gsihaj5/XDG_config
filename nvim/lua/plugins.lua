return require('packer').startup(function()
	-- packer to update itself
	use 'wbthomason/packer.nvim'

  --config files for built in lsp client
  use 'neovim/nvim-lspconfig'

end)
