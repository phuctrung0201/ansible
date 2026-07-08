return {
  {
    "obsidian-nvim/obsidian.nvim",
    lazy = false,
    dependencies = { "nvim-lua/plenary.nvim" },
    keys = {
      { "<leader>oo", "<cmd>Obsidian today<cr>", desc = "Obsidian: today" },
      { "<leader>oy", "<cmd>Obsidian yesterday<cr>", desc = "Obsidian: yesterday" },
      { "<leader>ot", "<cmd>Obsidian tomorrow<cr>", desc = "Obsidian: tomorrow" },
      { "<leader>on", "<cmd>Obsidian new<cr>", desc = "Obsidian: new note" },
      { "<leader>oN", "<cmd>Obsidian new_from_template<cr>", desc = "Obsidian: new from template" },
      { "<leader>op", "<cmd>Obsidian quick_switch<cr>", desc = "Obsidian: pick note" },
      { "<leader>os", "<cmd>Obsidian search<cr>", desc = "Obsidian: search" },
      { "<leader>ob", "<cmd>Obsidian backlinks<cr>", desc = "Obsidian: backlinks" },
      { "<leader>oi", "<cmd>Obsidian links<cr>", desc = "Obsidian: links in note" },
      { "<leader>or", "<cmd>Obsidian rename<cr>", desc = "Obsidian: rename note" },
      { "<leader>oT", "<cmd>Obsidian tags<cr>", desc = "Obsidian: tags" },
      { "<leader>oO", "<cmd>Obsidian open<cr>", desc = "Obsidian: open in app" },
      { "<leader>of", "<cmd>Obsidian follow_link<cr>", desc = "Obsidian: follow link" },
      { "<leader>oe", "<cmd>Obsidian template<cr>", desc = "Obsidian: insert template" },
    },
    opts = function()
      local builtin = require("obsidian.builtin")
      local vault = vim.env.OBSIDIAN_VAULT_DIR or "~/wiki"

      return {
        legacy_commands = false,
        workspaces = {
          { name = "wiki", path = vault },
        },
        notes_subdir = "notes",
        new_notes_location = "notes_subdir",
        note_id_func = builtin.title_id,
        sort_by = "modified",
        sort_reversed = true,

        note = {
          template = "note.md",
        },

        daily_notes = {
          folder = "journals",
          date_format = "%Y-%m-%d",
          alias_format = "%A, %B %-d %Y",
          template = "journal.md",
        },

        templates = {
          folder = ".templates",
          date_format = "%A, %B %-d %Y",
          time_format = "%H:%M",
        },

        completion = { min_chars = 2 },
        picker = { name = "snacks.pick" },

        link = {
          style = "wiki",
        },

        frontmatter = {
          enabled = true,
          func = function(note)
            local out = { tags = note.tags or {} }
            if note.metadata then
              for k, v in pairs(note.metadata) do
                out[k] = v
              end
            end
            return out
          end,
          sort = { "tags" },
        },

        mappings = {
          ["gf"] = {
            action = function()
              return require("obsidian").util.gf_passthrough()
            end,
            opts = { noremap = false, expr = true, buffer = true },
          },
          ["<cr>"] = {
            action = function()
              return require("obsidian").util.smart_action()
            end,
            opts = { buffer = true, expr = true },
          },
          ["<leader>ch"] = {
            action = function()
              return require("obsidian").util.toggle_checkbox()
            end,
            opts = { buffer = true },
          },
        },

        follow_url_func = vim.ui.open,
        follow_img_func = function(img)
          vim.fn.jobstart({ "qlmanage", "-p", img })
        end,

        ui = { enable = true },
      }
    end,
  },
}
