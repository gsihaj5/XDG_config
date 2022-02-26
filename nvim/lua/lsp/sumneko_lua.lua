local sumneko_bin_folder = "~/.local/share/nvim/lsp_servers/sumneko_lua/extension/server/bin/"
local sumneko_binary_path = sumneko_bin_folder .. "lua-language-server"
local sumneko_main_path = sumneko_bin_folder .. "main.lua"
local capabilities = require('cmp_nvim_lsp')
  .update_capabilities(vim.lsp.protocol.make_client_capabilities())

return require('lspconfig').sumneko_lua.setup{
    capabilities = capabilities;
    cmd = {sumneko_binary_path, "-E", sumneko_main_path};
  settings = {
    Lua = {
      runtime = {
        -- Tell the language server which version of Lua you're using (most likely LuaJIT in the case of Neovim)
        version = 'LuaJIT',
        -- Setup your lua path
        path = vim.split(package.path, ';'),
      },
      diagnostics = {
        -- Get the language server to recognize the `vim` global
        globals = {'vim'},
      },
      workspace = {
        -- Make the server aware of Neovim runtime files
        library = {
          [vim.fn.expand('$VIMRUNTIME/lua')] = true,
          [vim.fn.expand('$VIMRUNTIME/lua/vim/lsp')] = true,
        },
      },
    },
  },
}
