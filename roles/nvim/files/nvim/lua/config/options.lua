local opt = vim.opt

opt.number = true
opt.relativenumber = true
opt.cursorline = true
opt.cursorlineopt = "number"
opt.signcolumn = "yes"
opt.statuscolumn = "%s%=%{v:relnum ? v:relnum : v:lnum} "
opt.fillchars = { eob = " ", fold = "·", foldsep = " " }
opt.conceallevel = 2
opt.ignorecase = true
opt.spell = false
opt.wrap = false -- don't soft-wrap long lines by default
opt.foldenable = true
opt.foldlevelstart = 99 -- start with all folds open ("unfolded") by default
opt.foldmethod = "expr"
opt.foldexpr = "v:lua.vim.treesitter.foldexpr()"
opt.foldtext = ""

-- kickstart defaults
opt.mouse = "a"
opt.showmode = false -- mode is shown in mini.statusline
opt.smartcase = true
opt.breakindent = true
opt.undofile = true
opt.swapfile = false
opt.updatetime = 250
opt.timeoutlen = 300
opt.splitright = true
opt.splitbelow = true
opt.inccommand = "split"
opt.scrolloff = 10
opt.confirm = true
opt.list = true
opt.listchars = { tab = "  ", trail = "·", nbsp = "␣" }

vim.schedule(function() vim.o.clipboard = "unnamedplus" end)

vim.diagnostic.config({
  severity_sort = true,
  float = { border = "rounded", source = "if_many" },
  underline = { severity = { min = vim.diagnostic.severity.WARN } },
  virtual_text = true,
})
