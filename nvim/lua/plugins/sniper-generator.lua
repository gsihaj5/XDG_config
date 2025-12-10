return {
  dir = "~/projects/sniper-generator/",
  name = "sniper-generator",
  dependencies = { "MunifTanjim/nui.nvim", "nvim-lua/plenary.nvim" },
  config = function()
    require("sniper-generator").setup()
  end,
}
