require('plugins')
--plugin settings meant to settings mappings and properties for plugins
require('plugins-settings')

--lsp configuration
--require('lsp.denols')
require('lsp.tsserver')
require('lsp.efm')

require('lsp.psalm')
require('lsp.sumneko_lua')
require('lsp.intelephense')
require('lsp.tailwindcss')
require('lsp.jdtls')
require('lsp.pylsp')

--this is basic settings
require('settings')
require('mappings')
