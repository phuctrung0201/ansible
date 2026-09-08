-- Fuzzy finder (fzf-lua)
-- Picker toggles: <C-g> gitignored files, <C-h> hidden files, <C-S-g> grep mode.
return {
  "ibhagwan/fzf-lua",
  cmd = "FzfLua",
  event = "VimEnter",
  config = function()
    local fzf = require("fzf-lua")
    local actions = require("fzf-lua.actions")

    fzf.setup({
      winopts = {
        on_create = function()
          vim.keymap.set("t", "<C-S-g>", "<M-g>", { buffer = true })
        end,
      },
      actions = {
        files = {
          true, -- inherit default file actions (enter/ctrl-s/ctrl-v/ctrl-t/...)
          ["ctrl-g"] = actions.toggle_ignore,
          ["ctrl-h"] = actions.toggle_hidden,
        },
      },
      grep = {
        actions = {
          -- default file actions (enter/ctrl-s/ctrl-v/ctrl-t/...) are inherited
          -- automatically; do NOT add `true` here (breaks the `hide` profile).
          ["ctrl-g"] = actions.toggle_ignore,
          ["ctrl-h"] = actions.toggle_hidden,
          ["alt-g"] = actions.grep_lgrep,
        },
      },
      files = { cwd_prompt = false }, -- don't show cwd path in the files prompt
      oldfiles = { cwd_only = true }, -- scope recent files to cwd
    })

    -- Route vim.ui.select (code actions, rename, ...) through fzf-lua.
    fzf.register_ui_select()

    local map = vim.keymap.set
    map("n", "<leader>sh", fzf.help_tags, { desc = "Search help" })
    map("n", "<leader>sk", fzf.keymaps, { desc = "Search keymaps" })
    map("n", "<leader>sf", fzf.files, { desc = "Search files" })
    map("n", "<leader>ss", fzf.lsp_document_symbols, { desc = "Search symbols" })
    map("n", "<leader>sS", fzf.lsp_live_workspace_symbols, { desc = "Search workspace symbols" })
    map("n", "<leader>sw", fzf.grep_cword, { desc = "Search current word" })
    map("n", "<leader>sg", fzf.live_grep, { desc = "Search by grep" })
    map("n", "<leader>sd", fzf.diagnostics_workspace, { desc = "Search diagnostics" })
    map("n", "<leader>sq", fzf.quickfix, { desc = "Search quickfix" })
    map("n", "<leader>sr", fzf.resume, { desc = "Search resume" })
    map("n", "<leader>s.", fzf.oldfiles, { desc = "Search recent files" })
    map("n", "<leader><leader>", fzf.files, { desc = "Search files" })
    map("n", "<leader>sb", fzf.buffers, { desc = "Search buffers" })
  end,
}
