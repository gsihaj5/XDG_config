local eslint = {
  lintCommand = "eslint_d -f unix --stdin --stdin-filename ${INPUT}",
  lintStdin = true,
  lintFormats = {"%f:%l:%c: %m"},
  lintIgnoreExitCode = true,
  formatCommand = "eslint_d --fix-to-stdout --stdin --stdin-filename=${INPUT}",
  formatStdin = true
}
local efm_bin_folder = "/home/gerry/.local/share/nvim/lsp_servers/efm/"
local efm_binary_path = efm_bin_folder .. "efm-langserver"

return require('lspconfig').efm.setup {
	cmd = {efm_binary_path},
  	on_attach = function(client)
    	client.resolved_capabilities.document_formatting = true
  	end,
	root_dir = function()
    	return vim.fn.getcwd()
  	end,
  	settings = {
    	languages = {
      		javascript = {eslint},
      		javascriptreact = {eslint},
      		["javascript.jsx"] = {eslint},
      		typescript = {eslint},
      		["typescript.tsx"] = {eslint},
      		typescriptreact = {eslint}
    	}
  	},
  	filetypes = {
    	"javascript",
    	"javascriptreact",
    	"javascript.jsx",
    	"typescript",
    	"typescript.tsx",
    	"typescriptreact"
  	},
}
