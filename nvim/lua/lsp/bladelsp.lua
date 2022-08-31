local lspconfig = require'lspconfig'
local configs = require 'lspconfig.configs'

-- Configure it
configs.blade = {
  default_config = {
    -- Path to the executable: laravel-dev-generators
    cmd = { "/home/gerry/lsps/laravel-dev-tools-1.0.0/builds/laravel-dev-tools", "lsp" },
    filetypes = {'blade'};
    root_dir = function(fname)
      return lspconfig.util.find_git_ancestor(fname)
    end;
    settings = {};
  };
}
-- Set it up
lspconfig.blade.setup{
  -- Capabilities is specific to my setup.
  capabilities = capabilities
}
