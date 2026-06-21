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
      { "<leader>op", "<cmd>Obsidian quick_switch<cr>", desc = "Obsidian: pick note" },
      { "<leader>os", "<cmd>Obsidian search<cr>", desc = "Obsidian: search" },
      { "<leader>ob", "<cmd>Obsidian backlinks<cr>", desc = "Obsidian: backlinks" },
      { "<leader>oi", "<cmd>Obsidian links<cr>", desc = "Obsidian: links in note" },
      { "<leader>or", "<cmd>Obsidian rename<cr>", desc = "Obsidian: rename note" },
      { "<leader>oT", "<cmd>Obsidian tags<cr>", desc = "Obsidian: tags" },
    },
    opts = {
      legacy_commands = false,
      workspaces = {
        { name = "wiki", path = "~/wiki" },
      },
      notes_subdir = "",
      new_notes_location = "notes_subdir",
      note_id_func = function(title)
        if title == nil or title == "" then
          return "untitled"
        end
        return title:lower():gsub("[^%w]+", "_"):gsub("^_+", ""):gsub("_+$", "")
      end,
      daily_notes = {
        folder = "journals",
        date_format = "%Y-%m-%d",
        template = "journal.md",
      },
      templates = {
        folder = "templates",
        date_format = "%A, %B %-d %Y",
        time_format = "%H:%M",
      },
      completion = {
        min_chars = 2,
      },
      picker = {
        name = "snacks.pick",
      },
      link = {
        style = "wiki",
      },
      frontmatter = {
        enabled = false,
      },
      ui = { enable = true },
    },
  },
}
