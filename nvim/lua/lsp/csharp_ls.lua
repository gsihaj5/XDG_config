local csharp_ls_folder = "/home/gerry/.local/share/nvim/lsp_servers/csharp_ls"
local csharp_ls_binary_path = csharp_ls_folder .. "/csharp-ls"
local capabilities = require('cmp_nvim_lsp')
	.default_capabilities(vim.lsp.protocol.make_client_capabilities())


return require('lspconfig').csharp_ls.setup {
	cmd = { csharp_ls_binary_path },
	capabilities = capabilities
}
