return {
  {
    "echasnovski/mini.pairs",
    event = "InsertEnter",
    opts = {},
  },

  {
    "echasnovski/mini.comment",
    event = "BufReadPost",
    opts = {},
  },

  {
    "folke/flash.nvim",
    keys = {
      { "s", mode = { "n", "x", "o" }, function() require("flash").jump() end, desc = "Flash" },
      { "S", mode = { "n", "x", "o" }, function() require("flash").treesitter() end, desc = "Flash treesitter" },
    },
    opts = {},
  },
}
