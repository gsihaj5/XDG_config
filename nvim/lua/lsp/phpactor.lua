local capabilities = require('cmp_nvim_lsp')
  .default_capabilities(vim.lsp.protocol.make_client_capabilities())
local phpactor_bin_folder= "/home/gerry/.local/share/nvim/lsp_servers/phpactor-source/bin"
local phpactor_executable = phpactor_bin_folder .. '/phpactor'

return require('lspconfig').phpactor.setup{
    capabilities = capabilities,
 	cmd = {phpactor_executable}
}
