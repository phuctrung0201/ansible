-- mini.nvim: consolidates statusline, dashboard, autopairs, textobjects,
-- indent guides, file explorer, git signs/commands, icons, key hints, notifications.
return {
  "nvim-mini/mini.nvim",
  version = false,
  config = function()
    require("mini.icons").setup()
    MiniIcons.mock_nvim_web_devicons()

    require("mini.ai").setup({ n_lines = 500 })
    require("mini.pairs").setup()

    -- Surround under `gs` prefix (keeps `s` free for flash.nvim)
    require("mini.surround").setup({
      mappings = {
        add = "gsa",
        delete = "gsd",
        find = "gsf",
        find_left = "gsF",
        highlight = "gsh",
        replace = "gsr",
        update_n_lines = "gsn",
      },
    })

    require("mini.indentscope").setup()

    require("mini.git").setup()
    require("mini.diff").setup()
    vim.keymap.set("n", "<leader>gd", function()
      require("mini.diff").toggle_overlay()
    end, { desc = "Toggle diff overlay" })
    vim.keymap.set({ "n", "x" }, "<leader>gl", function()
      require("mini.git").show_range_history()
    end, { desc = "Git line history" })
    vim.keymap.set({ "n", "x" }, "<leader>gi", function()
      require("mini.git").show_at_cursor()
    end, { desc = "Git info at cursor" })

    -- Stage / reset hunks (mini.diff "apply" == git stage)
    local function hunk_range()
      local l1 = vim.fn.line(".")
      local l2 = vim.fn.line("v")
      return math.min(l1, l2), math.max(l1, l2)
    end
    vim.keymap.set({ "n", "x" }, "<leader>gs", function()
      local s, e = hunk_range()
      require("mini.diff").do_hunks(0, "apply", { line_start = s, line_end = e })
    end, { desc = "Stage hunk" })
    vim.keymap.set({ "n", "x" }, "<leader>gr", function()
      local s, e = hunk_range()
      require("mini.diff").do_hunks(0, "reset", { line_start = s, line_end = e })
    end, { desc = "Reset hunk" })
    vim.keymap.set("n", "<leader>gS", function()
      require("mini.diff").do_hunks(0, "apply")
    end, { desc = "Stage buffer" })
    vim.keymap.set("n", "<leader>gR", function()
      require("mini.diff").do_hunks(0, "reset")
    end, { desc = "Reset buffer" })
    vim.keymap.set("n", "<leader>gu", "<cmd>Git reset --quiet -- %<cr>", { desc = "Unstage file" })

    local starter = require("mini.starter")
    starter.setup({
      footer = "",
      items = {
        {
          name = "Explore",
          action = function()
            local name = vim.api.nvim_buf_get_name(0)
            local reveal = name ~= "" and vim.uv.fs_stat(name) ~= nil
            require("neo-tree.command").execute({ toggle = true, reveal = reveal })
          end,
          section = "Actions",
        },
        { name = "Recent files", action = "FzfLua oldfiles", section = "Actions" },
        { name = "Find files", action = "FzfLua files", section = "Actions" },
        { name = "Grep", action = "FzfLua live_grep", section = "Actions" },
        { name = "Lazygit", action = require("config.util").lazygit, section = "Actions" },
        { name = "Quit", action = "qall", section = "Actions" },
      },
    })

    require("mini.statusline").setup({
      use_icons = true,
      content = {
        active = function()
          local mode, mode_hl = MiniStatusline.section_mode({ trunc_width = 120 })
          local git = MiniStatusline.section_git({ trunc_width = 40 })
          local diff = MiniStatusline.section_diff({ trunc_width = 75 })
          local diagnostics = MiniStatusline.section_diagnostics({ trunc_width = 75 })
          local lsp = MiniStatusline.section_lsp({ trunc_width = 75 })
          local filename = MiniStatusline.section_filename({ trunc_width = 140 })
          return MiniStatusline.combine_groups({
            { hl = mode_hl, strings = { mode } },
            "%<",
            { hl = "MiniStatuslineFilename", strings = { filename } },
            "%=",
            { hl = "MiniStatuslineDevinfo", strings = { diff, diagnostics, lsp, git } },
          })
        end,
      },
    })

    require("mini.notify").setup({ lsp_progress = { enable = true } })
    vim.notify = require("mini.notify").make_notify()
    vim.keymap.set("n", "<leader>n", function()
      require("mini.notify").show_history()
    end, { desc = "Notification history" })

    local clue = require("mini.clue")
    clue.setup({
      triggers = {
        { mode = "n", keys = "<Leader>" },
        { mode = "x", keys = "<Leader>" },
        { mode = "n", keys = "g" },
        { mode = "x", keys = "g" },
        { mode = "n", keys = "'" },
        { mode = "n", keys = "`" },
        { mode = "n", keys = '"' },
        { mode = "i", keys = "<C-r>" },
        { mode = "n", keys = "<C-w>" },
        { mode = "n", keys = "z" },
        { mode = "x", keys = "z" },
      },
      clues = {
        clue.gen_clues.builtin_completion(),
        clue.gen_clues.g(),
        clue.gen_clues.marks(),
        clue.gen_clues.registers(),
        clue.gen_clues.windows({ submode_move = true, submode_navigate = true, submode_resize = true }),
        clue.gen_clues.z(),
        -- Global leader groups are centralized here (mini.clue sets clues once
        -- at setup); buffer-local groups live with their ftplugin/plugin.
        { mode = "n", keys = "<Leader>s", desc = "+Search" },
        { mode = "n", keys = "<Leader>d", desc = "+Database" },
        { mode = "n", keys = "<Leader>g", desc = "+Git" },
        { mode = "n", keys = "<Leader>b", desc = "+Buffer" },
        -- Other groups
        { mode = "n", keys = "gs", desc = "+Surround" },
        { mode = "x", keys = "gs", desc = "+Surround" },
      },
      window = { delay = 0 },
    })
  end,
}
