require('nvim_comment').setup()

vim.keymap.set("n", ",/", vim.cmd.CommentToggle)
