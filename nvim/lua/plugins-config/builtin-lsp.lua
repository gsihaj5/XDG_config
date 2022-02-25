local function map(mode, shortcut, command)
	vim.api.nvim_set_keymap(mode, shortcut, command, { noremap = true, silent = true})
end

local function nmap(shortcut, command)
	map('n', shortcut, command)
end

	nmap('<space>e', '<cmd>lua vim.diagnostic.open_float()<CR>')
	nmap('[d', '<cmd>lua vim.diagnostic.goto_prev()<CR>')
	nmap(']d', '<cmd>lua vim.diagnostic.goto_next()<CR>')
	nmap('<space>q', '<cmd>lua vim.diagnostic.setloclist()<CR>')
	nmap('gD', '<cmd>lua vim.lsp.buf.declaration()<CR>')
	nmap('gd', '<cmd>lua vim.lsp.buf.definition()<CR>')
	nmap('K', '<cmd>lua vim.lsp.buf.hover()<CR>')
	nmap('gi', '<cmd>lua vim.lsp.buf.implementation()<CR>')
	nmap('<C-K>', '<cmd>lua vim.lsp.buf.signature_help()<CR>')
	nmap('<space>D', '<cmd>lua vim.lsp.buf.type_definition()<CR>')
	nmap('<space>rn', '<cmd>lua vim.lsp.buf.rename()<CR>')
	nmap('<space>ca', '<cmd>lua vim.lsp.buf.code_action()<CR>')
	nmap('gr', '<cmd>lua vim.lsp.buf.references()<CR>')
	nmap('<C-A-l>', '<cmd>lua vim.lsp.buf.formatting()<CR>')
