local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())

return require('lspconfig').intelephense.setup{
    capabilities = capabilities
      --on_attach = function(client)
        --client.resolved_capabilities.document_formatting = false
      --end,
}
