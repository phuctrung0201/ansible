local opt = vim.opt

opt.number = true
opt.relativenumber = true
opt.cursorline = true
opt.cursorlineopt = "number"
opt.signcolumn = "number"
opt.statuscolumn = "%s%=%{v:relnum ? v:relnum : v:lnum} "
opt.fillchars = {
  eob = " ",
  fold = "·",
  foldsep = " ",
}
opt.conceallevel = 2
opt.ignorecase = true

opt.foldenable = true
opt.foldmethod = "expr"
opt.foldexpr = "v:lua.vim.treesitter.foldexpr()"
opt.foldtext = ""
