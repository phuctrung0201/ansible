return {
  {
    "obsidian-nvim/obsidian.nvim",
    lazy = false,
    dependencies = { "nvim-lua/plenary.nvim" },
    keys = {
      { "<leader>ot", "<cmd>Obsidian today<cr>", desc = "Obsidian: today" },
      { "<leader>oy", "<cmd>Obsidian yesterday<cr>", desc = "Obsidian: yesterday" },
      { "<leader>on", "<cmd>Obsidian new<cr>", desc = "Obsidian: new note" },
      { "<leader>oN", "<cmd>Obsidian new_from_template<cr>", desc = "Obsidian: new from template" },
      { "<leader>op", "<cmd>Obsidian quick_switch<cr>", desc = "Obsidian: pick note" },
      { "<leader>os", "<cmd>Obsidian search<cr>", desc = "Obsidian: search" },
      { "<leader>ob", "<cmd>Obsidian backlinks<cr>", desc = "Obsidian: backlinks" },
      { "<leader>oi", "<cmd>Obsidian links<cr>", desc = "Obsidian: links in note" },
      { "<leader>or", "<cmd>Obsidian rename<cr>", desc = "Obsidian: rename note" },
      { "<leader>oT", "<cmd>Obsidian tags<cr>", desc = "Obsidian: tags" },
      { "<leader>of", "<cmd>Obsidian follow_link<cr>", desc = "Obsidian: follow link" },
      { "<leader>oe", "<cmd>Obsidian template<cr>", desc = "Obsidian: insert template" },
    },
    opts = function()
      return {
        legacy_commands = false,
        link = {
          style = "markdown",
          format = "shortest",
        },
        workspaces = {
          { name = "wiki", path = "~/wiki" },
        },
        notes_subdir = "notes",
        new_notes_location = "notes_subdir",
        note_id_func = require("obsidian.builtin").title_id,
        note = { template = "note.md" },
        daily_notes = {
          folder = "journals",
          date_format = "%Y-%m-%d",
          alias_format = "%A, %B %-d %Y",
          template = "journal.md",
        },
        templates = {
          folder = "_templates",
          date_format = "%A, %B %-d %Y",
          time_format = "%H:%M",
        },
        picker = { name = "snacks.pick" },
        frontmatter = { enabled = false },
      }
    end,
    config = function(_, opts)
      require("obsidian").setup(opts)

      vim.api.nvim_create_autocmd("FileType", {
        pattern = "markdown",
        callback = function(args)
          vim.keymap.set(
            "n",
            "<leader>oo",
            "<cmd>Obsidian open<cr>",
            { buffer = args.buf, desc = "Obsidian: open in app" }
          )
        end,
      })
    end,
  },

  {
    "folke/which-key.nvim",
    opts = {
      spec = {
        { "<leader>o", group = "Obsidian", icon = "󱓧" },
      },
    },
  },
}
