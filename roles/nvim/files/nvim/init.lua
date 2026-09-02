-- Neovim configuration (kickstart.nvim-based, managed by ansible role `nvim`).
-- Modular layout:
--   lua/config/*  -> core settings (options, keymaps, autocmds, theme, lazy bootstrap)
--   lua/plugins/* -> one file per plugin spec (auto-imported by lazy.nvim)
-- Theme colors come from lua/config/env.lua (ansible-templated to match tmux).

vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

require("config.options")
require("config.keymaps")
require("config.autocmds")

local theme = require("config.theme")
vim.api.nvim_create_autocmd("ColorScheme", { callback = theme.apply })

require("config.lazy")

-- Apply theme highlights after all start plugins are loaded.
theme.apply()
