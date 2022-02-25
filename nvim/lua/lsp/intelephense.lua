local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())
local on_attach = require('lsp.mappings_on_attach')

return require('lspconfig').intelephense.setup{
    capabilities = capabilities,
    on_attach = on_attach
}
