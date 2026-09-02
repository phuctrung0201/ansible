-- Completion
return {
  "saghen/blink.cmp",
  version = "1.*",
  dependencies = {
    { "L3MON4D3/LuaSnip", version = "2.*" },
  },
  opts = {
    keymap = { preset = "default" },
    appearance = { nerd_font_variant = "mono" },
    completion = { documentation = { auto_show = false } },
    sources = {
      default = { "lsp", "path", "snippets" },
      per_filetype = {
        sql = { "dadbod", "lsp", "path", "snippets" },
        mysql = { "dadbod", "lsp", "path", "snippets" },
        plsql = { "dadbod", "lsp", "path", "snippets" },
      },
      providers = {
        dadbod = { name = "Dadbod", module = "vim_dadbod_completion.blink" },
      },
    },
    snippets = { preset = "luasnip" },
    fuzzy = { implementation = "lua" },
    signature = { enabled = true },
  },
}
