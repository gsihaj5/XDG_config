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
-- nmap('<C-n><C-t>', ':tabnew<cr>')
-- nmap('<C-c><C-t>', ':tabclose<cr>')

-- nmap('<C-n><Tab>', ':tabn<cr>')
nmap('<C-p><Tab>', ':tabp<cr>')

-- clear search highlight
nmap('<space>,', ':let @/ = ""<cr>')

-- commenter
nmap(',/', ':call nerdcommenter#Comment(0, "toggle")<CR>')

-- git
nmap('<space>gs', ':G<CR>')
nmap('<space>gq', ':Gdiff :0<CR>')
nmap('<space>gh', ':diffget //2<CR>')
nmap('<space>gl', ':diffget //3<CR>')
nmap('<space>gc', ':Git commit<CR>')
nmap('<space>gp', ':Git push<CR>')

-- harpoon
nmap('<C-q>', ":lua require('harpoon.ui').toggle_quick_menu()<CR>")
nmap('<C-p>', ":lua require('harpoon.mark').add_file()<CR>")
