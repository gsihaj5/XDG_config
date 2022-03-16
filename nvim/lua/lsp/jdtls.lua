local jdtls_bin_folder = "/home/gerry/.local/share/nvim/lsp_servers/jdtls/bin/"
local jdtls_binary_path = jdtls_bin_folder .. "jdtls"
local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())

return require('lspconfig').jdtls.setup{
    capabilities = capabilities,
	cmd={jdtls_binary_path},
	on_attach = function(...)
      require'vim.lsp.log'.error('xxx on_attach: '..vim.inspect(...))
    end,
    on_exit = function(...)
      require'vim.lsp.log'.error('xxx on_exit: '..vim.inspect(...))
    end,
  	filetypes = {
		"java"
  	},
}
