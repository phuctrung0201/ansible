vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

local opt = vim.opt

opt.number = true
opt.relativenumber = true

opt.ignorecase = true
opt.smartcase = true
opt.hlsearch = true
opt.incsearch = true

opt.expandtab = true
opt.shiftwidth = 2
opt.tabstop = 2
opt.smartindent = true
opt.shiftround = true

opt.termguicolors = true
opt.signcolumn = "yes"
opt.cursorline = true
opt.scrolloff = 8
opt.sidescrolloff = 8

opt.splitright = true
opt.splitbelow = true

opt.clipboard = "unnamedplus"

opt.undofile = true
opt.swapfile = false
opt.backup = false

opt.completeopt = "menu,menuone,noselect"

opt.wrap = false
opt.showmode = false
opt.updatetime = 200
opt.timeoutlen = 300
