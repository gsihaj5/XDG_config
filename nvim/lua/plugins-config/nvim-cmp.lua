-- Setup nvim-cmp.
local cmp = require'cmp'
local lspkind = require "lspkind"
lspkind.init()

  cmp.setup({
    snippet = {
      expand = function(args)
        require('luasnip').lsp_expand(args.body) -- For `luasnip` users.
      end,
    },
    mapping = {
      ['<C-b>'] = cmp.mapping(cmp.mapping.scroll_docs(-4), { 'i', 'c' }),
      ['<C-f>'] = cmp.mapping(cmp.mapping.scroll_docs(4), { 'i', 'c' }),
      ['<C-Space>'] = cmp.mapping(cmp.mapping.complete(), { 'i', 'c' }),
      ['<C-y>'] = cmp.config.disable, -- Specify `cmp.config.disable` if you want to remove the default `<C-y>` mapping.
      ['<C-e>'] = cmp.mapping({
        i = cmp.mapping.abort(),
        c = cmp.mapping.close(),
      }),
	  ['<C-n>'] = function(fallback)
	   	if cmp.visible() then
		 	cmp.select_next_item()
	   	else
		 	fallback()
	   	end
	  end,
	  ['<C-p>'] = function(fallback)
	   	if cmp.visible() then
		 	cmp.select_prev_item()
	   	else
		 	fallback()
	   	end
	  end,
      ['<CR>'] = cmp.mapping.confirm({ select = true }), -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
    },
    sources = cmp.config.sources({
      { name = 'nvim_lsp' },
	  { name = 'luasnip' }, -- For luasnip users.
      { name = 'path' },
      { name = 'buffer', keyword_length = 5 },
    }),
	formatting = {
		format = lspkind.cmp_format {
			with_text = true,
      		menu = {
        		nvim_lsp = "[LSP]",
        		luasnip = "[snip]",
        		path = "[path]",
        		buffer = "[buf]",
      		},
		}
	}
  })

  -- Set configuration for specific filetype.
  cmp.setup.filetype('gitcommit', {
    sources = cmp.config.sources({
      { name = 'cmp_git' }, -- You can specify the `cmp_git` source if you were installed it. 
    }, {
      { name = 'buffer' },
    })
  })

  -- Use buffer source for `/` (if you enabled `native_menu`, this won't work anymore).
  cmp.setup.cmdline('/', {
    sources = {
      { name = 'buffer' }
    }
  })

  -- Use cmdline & path source for ':' (if you enabled `native_menu`, this won't work anymore).
  cmp.setup.cmdline(':', {
    sources = cmp.config.sources({
      { name = 'path' }
    }, {
      { name = 'cmdline' }
    })
  })
