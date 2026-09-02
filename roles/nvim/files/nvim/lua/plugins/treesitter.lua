-- Treesitter (main branch)
return {
  "nvim-treesitter/nvim-treesitter",
  branch = "main",
  lazy = false,
  build = ":TSUpdate",
  config = function()
    local parsers = {
      "bash", "c", "diff", "lua", "luadoc", "markdown", "markdown_inline",
      "query", "vim", "vimdoc", "c_sharp", "fish", "sql", "go", "gomod",
      "gosum", "python", "typescript", "tsx", "javascript", "rust", "zig",
      "json",
    }
    require("nvim-treesitter").install(parsers)

    local function try_attach(buf, language)
      if not vim.treesitter.language.add(language) then return end
      vim.treesitter.start(buf, language)
      if vim.treesitter.query.get(language, "indents") ~= nil then
        vim.bo[buf].indentexpr = "v:lua.require'nvim-treesitter'.indentexpr()"
      end
    end

    local available = require("nvim-treesitter").get_available()
    vim.api.nvim_create_autocmd("FileType", {
      group = vim.api.nvim_create_augroup("treesitter_attach", { clear = true }),
      callback = function(args)
        local buf, filetype = args.buf, args.match
        local language = vim.treesitter.language.get_lang(filetype)
        if not language then return end
        local installed = require("nvim-treesitter").get_installed("parsers")
        if vim.tbl_contains(installed, language) then
          try_attach(buf, language)
        elseif vim.tbl_contains(available, language) then
          require("nvim-treesitter").install(language):await(function() try_attach(buf, language) end)
        else
          try_attach(buf, language)
        end
      end,
    })
  end,
}
