local capabilities = require('cmp_nvim_lsp')
  .default_capabilities(vim.lsp.protocol.make_client_capabilities())
local jdtls_folder = "/home/gerry/.local/share/nvim/lsp_servers/pylsp/"
local jdtls_bin_folder = jdtls_folder .. "venv/bin/pylsp"

return require('lspconfig').pylsp.setup{
	capabilities = capabilities,
	cmd = {jdtls_bin_folder}
	
}
