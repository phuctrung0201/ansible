vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

require("config.options")
require("config.pack")
require("config.mini")
require("config.lsp")
require("config.format")
require("config.keymaps")
require("config.autocmds")

require("plugins.catppuccin")
require("plugins.treesitter")
require("plugins.obsidian")
require("plugins.sql")
require("plugins.git")
