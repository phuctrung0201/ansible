return {
  "obsidian-nvim/obsidian.nvim",
  version = "*",
  ft = { "markdown" },
  dependencies = {
    "ibhagwan/fzf-lua",
  },
  opts = function()
    local theme = require("config.env")

    return {
      legacy_commands = false,
      workspaces = {
        {
          name = "wiki",
          path = "~/wiki",
        },
      },
      picker = {
        name = "fzf-lua",
        note_mappings = {
          new = "",
          insert_link = "",
          bookmark = "",
        },
      },
      frontmatter = {
        enabled = false,
      },
      ui = {
        enable = true,
        hl_groups = {
          ObsidianTodo = { bold = true, fg = theme.peach },
          ObsidianDone = { bold = true, fg = theme.green },
          ObsidianRightArrow = { bold = true, fg = theme.peach },
          ObsidianTilde = { bold = true, fg = theme.yellow },
          ObsidianImportant = { bold = true, fg = theme.red },
          ObsidianBullet = { bold = true, fg = theme.peach },
          ObsidianRefText = { underline = true, fg = theme.blue },
          ObsidianExtLinkIcon = { fg = theme.teal },
          ObsidianTag = { italic = true, fg = theme.mauve },
          ObsidianBlockID = { italic = true, fg = theme.lavender },
          ObsidianHighlightText = { bg = theme.surface1 },
        },
      },
      callbacks = {
        enter_note = function()
          local map = function(mode, lhs, rhs, desc)
            vim.keymap.set(mode, lhs, rhs, { buffer = true, desc = desc })
          end

          map("n", "<CR>", "<cmd>Obsidian follow_link<cr>", "Follow Obsidian link")
          map("n", "<leader>ob", "<cmd>Obsidian backlinks<cr>", "Backlinks")
          map("n", "<leader>ol", "<cmd>Obsidian links<cr>", "Links")
          map("n", "<leader>oo", "<cmd>Obsidian open<cr>", "Open in Obsidian")

          vim.b.miniclue_config = {
            clues = {
              { mode = "n", keys = "<Leader>o", desc = "+Obsidian" },
            },
          }
        end,
      },
    }
  end,
  config = function(_, opts)
    require("obsidian").setup(opts)

    local fzf = require("fzf-lua")
    if fzf._obsidian_prompt_in_title then
      return
    end

    local fzf_exec = fzf.fzf_exec
    fzf.fzf_exec = function(contents, picker_opts)
      if picker_opts and type(picker_opts.prompt) == "string" then
        local title = picker_opts.prompt:match("^(.-) | <CR> confirm")
        if title then
          picker_opts = vim.deepcopy(picker_opts)
          picker_opts.prompt = "❯ "
          picker_opts.winopts = vim.tbl_deep_extend("force", picker_opts.winopts or {}, {
            title = " " .. title .. " ",
            title_pos = "center",
          })
        end
      end

      return fzf_exec(contents, picker_opts)
    end
    fzf._obsidian_prompt_in_title = true
  end,
}
