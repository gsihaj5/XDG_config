local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())
local psalm_bin_folder= "/home/gerry/.local/share/nvim/lsp_servers/psalm/vendor/bin/"
local psalm_cmd = psalm_bin_folder .. 'psalm-language-server'
local psalm_executable = psalm_bin_folder .. 'psalm'

return require('lspconfig').psalm.setup{
    capabilities = capabilities,
    cmd = {psalm_cmd, "-E", psalm_executable};
}
