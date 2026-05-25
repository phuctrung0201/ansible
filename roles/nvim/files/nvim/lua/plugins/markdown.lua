return {
  {
    "MeanderingProgrammer/render-markdown.nvim",
    dependencies = { "nvim-treesitter/nvim-treesitter", "nvim-tree/nvim-web-devicons" },
    ft = { "markdown" },
    opts = {
      file_types = { "markdown" },
      heading = { sign = false, width = "block" },
      code = { sign = false, width = "block", right_pad = 1 },
      checkbox = {
        unchecked = { icon = "󰄱 " },
        checked = { icon = "󰱒 " },
      },
    },
    init = function()
      vim.opt.conceallevel = 2
    end,
  },
}
