local capabilities = require('cmp_nvim_lsp')
  .default_capabilities(vim.lsp.protocol.make_client_capabilities())

return require('lspconfig').denols.setup{
 capabilities = capabilities,
}
