return {
    'echasnovski/mini.splitjoin',
    config = function(_, opts)
        require('mini.splitjoin').setup({
            mappings = {
                toggle = 'tp',
                split = '',
                join = '',
            },
        })
    end

}
