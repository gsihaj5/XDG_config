local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())

local vuels_bin_folder = "/home/gerry/.local/share/nvim/lsp_servers/vuels/node_modules/vls/bin"
local vuels_binary_path = vuels_bin_folder.. "/vls"

return require('lspconfig').vuels.setup{
  	capabilities = capabilities,
  	cmd = {vuels_binary_path},
}
