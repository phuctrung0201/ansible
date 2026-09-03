-- Markdown in-buffer rendering, link navigation, and browser preview
return {
  -- In-buffer rendering + smart link navigation. `gx` (and <CR>, mapped in
  -- config/autocmds.lua) follows headings, files, Obsidian internal links,
  -- and URLs. `prefer_nvim` opens text files inside Neovim instead of the OS.
  {
    "OXY2DEV/markview.nvim",
    ft = { "markdown", "markdown.mdx" },
    opts = {
      preview = {
        map_gx = true,
        hybrid_modes = { "n" },
      },
      experimental = {
        prefer_nvim = true,
      },
    },
  },

  -- Live browser preview with synced scrolling. Keymaps live in the
  -- `<leader>m` Markdown group (see FileType autocmd in config/autocmds.lua).
  {
    "iamcco/markdown-preview.nvim",
    ft = { "markdown", "markdown.mdx" },
    cmd = { "MarkdownPreview", "MarkdownPreviewStop", "MarkdownPreviewToggle" },
    build = function() vim.fn["mkdp#util#install"]() end,
    init = function()
      vim.g.mkdp_auto_close = 0
    end,
  },
}
