-- mapping function to connect to vim api
function map(mode, shortcut, command)
	vim.api.nvim_set_keymap(mode, shortcut, command, { noremap = true, silent = true})
end

function nmap(shortcut, command)
	map('n', shortcut, command)
end

function imap(shortcut, command)
	map('i', shortcut, command)
end

function vmap(shortcut, command)
	map('v', shortcut, command)
end

function cmap(shortcut, command)
	map('c', shortcut, command)
end

function tmap(shortcut, command)
	map('t', shortcut, command)
end

-- THE MAPPINGS

-- keep search result in the midle of window
nmap('n', 'nzzzv')
nmap('N', 'Nzzzv')


-- WINDOWS
-- opening new window
-- split horizontal
nmap('sh', ':sp<cr>')
-- split vertical
nmap('sv', ':vs<cr>')

-- window navigation
nmap('<C-h>', '<C-w>h')
nmap('<C-j>', '<C-w>j')
nmap('<C-k>', '<C-w>k')
nmap('<C-l>', '<C-w>l')

-- window resize
nmap('<A-k>', ':res -1<cr>')
nmap('<A-j>', ':res +1<cr>')
nmap('<A-h>', ':vert res -1<cr>')
nmap('<A-l>', ':vert res +1<cr>')

-- TABS
nmap('<C-n><C-t>', ':tabnew<cr>')
nmap('<C-c><C-t>', ':tabclose<cr>')

nmap('<C-n><Tab>', ':tabn<cr>')
nmap('<C-p><Tab>', ':tabp<cr>')

vim.g.mapleader = " "

nmap('<space>e', '<cmd>lua vim.diagnostic.open_float()<CR>')
nmap('[d', '<cmd>lua vim.diagnostic.goto_prev()<CR>')
nmap(']d', '<cmd>lua vim.diagnostic.goto_next()<CR>')
nmap('<space>q', '<cmd>lua vim.diagnostic.setloclist()<CR>')
