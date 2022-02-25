local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())
return require('lspconfig').psalm.setup{
 capabilities = capabilities
}
