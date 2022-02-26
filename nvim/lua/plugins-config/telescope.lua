local function map(mode, shortcut, command)
	vim.api.nvim_set_keymap(mode, shortcut, command, { noremap = true, silent = true})
end

local function nmap(shortcut, command)
	map('n', shortcut, command)
end

--Using Lua functions
nmap("<space>ff", "<cmd>lua require('telescope.builtin').find_files()<cr>")
nmap("<space>fg", "<cmd>lua require('telescope.builtin').live_grep()<cr>")
nmap("<space>fb", "<cmd>lua require('telescope.builtin').buffers()<cr>")
nmap("<space>fh", "<cmd>lua require('telescope.builtin').help_tags()<cr>")
