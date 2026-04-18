local opt = vim.opt

-- Deltas from LazyVim defaults
opt.spell = false
opt.number = true
opt.relativenumber = true
opt.showtabline = 2
opt.wrap = true
opt.scrolloff = 8
opt.tabstop = 2
opt.shiftround = true

vim.filetype.add({
  extension = { zsh = "sh" },
  filename = { [".zshrc"] = "sh", [".zshenv"] = "sh", [".zprofile"] = "sh" },
})
