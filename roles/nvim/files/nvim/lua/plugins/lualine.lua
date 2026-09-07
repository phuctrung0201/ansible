return {
  "nvim-lualine/lualine.nvim",
  dependencies = { "nvim-tree/nvim-web-devicons" },
  config = function()
    local colors = require("config.env")
    local section_b = { fg = colors.text, bg = colors.surface0 }
    local section_c = { fg = colors.text, bg = colors.crust }
    local theme = {
      normal = {
        a = { fg = colors.crust, bg = colors.green, gui = "bold" },
        b = section_b,
        c = section_c,
      },
      insert = {
        a = { fg = colors.crust, bg = colors.teal, gui = "bold" },
        b = section_b,
        c = section_c,
      },
      visual = {
        a = { fg = colors.crust, bg = colors.mauve, gui = "bold" },
        b = section_b,
        c = section_c,
      },
      replace = {
        a = { fg = colors.crust, bg = colors.pink, gui = "bold" },
        b = section_b,
        c = section_c,
      },
      command = {
        a = { fg = colors.crust, bg = colors.peach, gui = "bold" },
        b = section_b,
        c = section_c,
      },
      inactive = {
        a = { fg = colors.overlay0, bg = colors.surface0 },
        b = { fg = colors.overlay0, bg = colors.surface0 },
        c = { fg = colors.overlay0, bg = colors.crust },
      },
    }

    require("lualine").setup({
      options = {
        theme = theme,
        component_separators = "",
        section_separators = "",
      },
      sections = {
        lualine_a = { "mode" },
        lualine_b = {},
        lualine_c = { "filename" },
        lualine_x = {},
        lualine_y = { "diff", "diagnostics", "lsp_status", "branch" },
        lualine_z = {},
      },
      inactive_sections = {
        lualine_a = {},
        lualine_b = {},
        lualine_c = { "filename" },
        lualine_x = {},
        lualine_y = {},
        lualine_z = {},
      },
    })
  end,
}
