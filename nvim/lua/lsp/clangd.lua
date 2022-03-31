local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())
local clangd_bin_folder= "/home/gerry/.local/share/nvim/lsp_servers/clangd/clangd/bin"
local clangd_executable = clangd_bin_folder .. '/clangd'

return require('lspconfig').clangd.setup{
 capabilities = capabilities,
 cmd = {clangd_executable}
}
