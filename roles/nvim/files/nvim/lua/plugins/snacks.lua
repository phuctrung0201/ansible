local colors = require("config.env")

return {
  "folke/snacks.nvim",
  priority = 1000,
  lazy = false,
  opts = {
    dashboard = {
      preset = {
        header = [[
█░░ █▀▀▀ ▀█▀ █▀▄▀█
█░░ █░▀█ ░█░ █░▀░█
█▄▄ █▄▄█ ░█░ █░░░█]],
        keys = {
          {
            icon = " ",
            key = "e",
            desc = "Explore",
            action = function()
              local name = vim.api.nvim_buf_get_name(0)
              local reveal = name ~= "" and vim.uv.fs_stat(name) ~= nil
              require("neo-tree.command").execute({ toggle = true, reveal = reveal })
            end,
          },
          { icon = " ", key = "r", desc = "Recent files", action = ":FzfLua oldfiles" },
          { icon = " ", key = "f", desc = "Find files", action = ":FzfLua files" },
          { icon = " ", key = "g", desc = "Grep", action = ":FzfLua live_grep" },
          {
            icon = " ",
            key = "l",
            desc = "Lazygit",
            action = function()
              Snacks.lazygit()
            end,
          },
          { icon = " ", key = "q", desc = "Quit", action = ":qall" },
        },
      },
      sections = {
        { section = "header" },
        { section = "keys", gap = 1, padding = 1 },
      },
    },
    git = {},
    gitbrowse = {},
    indent = {},
    lazygit = {
      config = {
        gui = {
          theme = {
            activeBorderColor = { colors.peach, "bold" },
            inactiveBorderColor = { colors.subtext0 },
            searchingActiveBorderColor = { colors.yellow },
            optionsTextColor = { colors.blue },
            selectedLineBgColor = { colors.surface0 },
            inactiveViewSelectedLineBgColor = { colors.overlay0 },
            cherryPickedCommitFgColor = { colors.blue },
            cherryPickedCommitBgColor = { colors.surface1 },
            markedBaseCommitFgColor = { colors.blue },
            markedBaseCommitBgColor = { colors.yellow },
            unstagedChangesColor = { colors.red },
            defaultFgColor = { colors.text },
          },
        },
      },
    },
    notifier = {},
    picker = {
      ui_select = false,
    },
    toggle = {},
  },
  init = function()
    local progress = {}
    vim.api.nvim_create_autocmd("LspProgress", {
      callback = function(event)
        local client = vim.lsp.get_client_by_id(event.data.client_id)
        local value = event.data.params.value
        if not client or type(value) ~= "table" then
          return
        end

        local id = client.id .. ":" .. tostring(event.data.params.token)
        if value.kind == "end" then
          progress[id] = nil
        else
          progress[id] = value
        end

        local messages = {}
        for _, item in pairs(progress) do
          local percentage = item.percentage and ("[%3d%%] "):format(item.percentage) or ""
          table.insert(messages, percentage .. (item.title or "") .. (item.message and " " .. item.message or ""))
        end
        if #messages > 0 then
          vim.notify(table.concat(messages, "\n"), vim.log.levels.INFO, {
            id = "lsp_progress",
            title = "LSP Progress",
          })
        end
      end,
    })
  end,
  config = function(_, opts)
    require("snacks").setup(opts)
    Snacks.toggle({
      name = "Auto Format (Buffer)",
      get = function()
        return vim.b.autoformat ~= false
      end,
      set = function(state)
        vim.b.autoformat = state
      end,
    }):map("<leader>tf")
  end,
  keys = {
    {
      "<leader>n",
      function()
        Snacks.notifier.show_history()
      end,
      desc = "Notification history",
    },
    {
      "<leader>gg",
      function()
        Snacks.lazygit()
      end,
      desc = "Lazygit",
    },
    {
      "<leader>gl",
      function()
        Snacks.git.blame_line()
      end,
      desc = "Git line history",
    },
    {
      "<leader>gi",
      function()
        Snacks.lazygit.log_file()
      end,
      desc = "Git file history",
    },
    {
      "<leader>gy",
      function()
        Snacks.gitbrowse({
          what = "permalink",
          open = function(url)
            vim.fn.setreg("+", url)
            vim.notify("Copied git permalink")
          end,
        })
      end,
      mode = { "n", "x" },
      desc = "Copy git permalink",
    },
    {
      "<leader>go",
      function()
        Snacks.gitbrowse({ what = "permalink" })
      end,
      mode = { "n", "x" },
      desc = "Open git permalink",
    },
    {
      "<leader>gu",
      function()
        Snacks.picker.git_status()
      end,
      desc = "Git status",
    },
  },
}
