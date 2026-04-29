local opt = vim.opt

-- Deltas from LazyVim defaults
opt.spell = false
opt.wrap = false
opt.scrolloff = 8
opt.tabstop = 2
opt.shiftround = true

vim.filetype.add({
  extension = { zsh = "sh" },
  filename = { [".zshrc"] = "sh", [".zshenv"] = "sh", [".zprofile"] = "sh" },
})
