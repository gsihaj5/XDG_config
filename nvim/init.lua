require('plugins')
--plugin settings meant to settings mappings and properties for plugins
require('plugins-settings')

--lsp configuration
--require('lsp.denols')
require('lsp.tsserver')
--require('lsp.efm')

require('lsp.vuels')
require('lsp.bladelsp')
require('lsp.omnisharp')
--require('lsp.csharp_ls')

-- php
require('lsp.psalm')
require('lsp.intelephense')
require('lsp.phpactor')

require('lsp.sumneko_lua')
require('lsp.tailwindcss')
require('lsp.jdtls')
require('lsp.pylsp')
require('lsp.clangd')

--this is basic settings
require('settings')
require('mappings')
