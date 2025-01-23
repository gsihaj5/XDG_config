-- Autocmds are automatically loaded on the VeryLazy event
-- Default autocmds that are always set: https://github.com/LazyVim/LazyVim/blob/main/lua/lazyvim/config/autocmds.lua
-- Add any additional autocmds here
--
--
--
-- auto save
vim.api.nvim_create_autocmd({ "InsertLeave", "TextChanged" }, {
  pattern = { "*" },
  command = "silent! wall",
  nested = true,
})

require("lspconfig").jdtls.setup({
    root_dir = function(fname)
        return require("lspconfig").util.root_pattern("pom.xml", "gradle.build", ".git")(fname) or vim.fn.getcwd()
    end,
    settings = {
        java = {
            format = {
                settings = {
                    -- url = function()
                    --     local root_dir = require("jdtls.setup").find_root(".git")
                    --     print("searching code_style.xml", root_dir)
                    --     if root_dir == nil then
                    --         return ''
                    --     else
                    --         return root_dir .. '/code_style.xml'
                    --     end
                    -- end
                }
            }
        }
    },
})
