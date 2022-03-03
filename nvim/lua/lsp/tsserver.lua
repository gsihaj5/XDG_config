local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())

local tsserver_bin_folder = "/home/gerry/.local/share/nvim/lsp_servers/tsserver/node_modules/.bin/"
local tsserver_binary_path = tsserver_bin_folder .. "typescript-language-server"


return require('lspconfig').tsserver.setup{
  	capabilities = capabilities,
  	cmd = {tsserver_binary_path, "--stdio"},
  	on_attach = function(client)
    	client.resolved_capabilities.document_formatting = false
  	end,
}
