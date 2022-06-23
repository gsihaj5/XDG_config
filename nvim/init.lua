require('plugins')
--plugin settings meant to settings mappings and properties for plugins
require('plugins-settings')

--lsp configuration
--require('lsp.denols')
require('lsp.tsserver')
--require('lsp.efm')

require('lsp.vuels')

--require('lsp.psalm')
require('lsp.sumneko_lua')
require('lsp.intelephense')
require('lsp.tailwindcss')
require('lsp.jdtls')
require('lsp.pylsp')
require('lsp.clangd')

--this is basic settings
require('settings')
require('mappings')
