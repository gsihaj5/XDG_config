local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())

local tailwind_bin_folder = "~/.local/share/nvim/lsp_servers/tailwindcss_npm/node_modules/@tailwindcss/language-server/bin/"
local tailwind_binary_path = tailwind_bin_folder .. "tailwindcss-language-server"
local tailwind_main_path = tailwind_bin_folder .. "main.lua"

return require('lspconfig').tailwindcss.setup{
    capabilities = capabilities,
	cmd = {tailwind_binary_path}
}
