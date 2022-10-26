--local omnisharp_folder = "/home/gerry/.local/share/nvim/lsp_servers/omnisharp/omnisharp"
--local omnisharp_binary_path = omnisharp_folder .. "/OmniSharp.dll"
local omnisharp_folder = "/home/gerry/.local/omnisharp"
local omnisharp_binary_path = omnisharp_folder .. "/run"
local capabilities = require('cmp_nvim_lsp')
	.update_capabilities(vim.lsp.protocol.make_client_capabilities())


return require('lspconfig').omnisharp.setup {
	--cmd = { "dotnet", omnisharp_binary_path },
	use_mono = true,
	mono_cmd = "/usr/bin/mono",
	cmd = { omnisharp_binary_path, "--languageserver", "--hostPID", tostring(vim.fn.getpid()) },
	capabilities = capabilities
}
