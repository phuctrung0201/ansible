require("flash").setup()

vim.keymap.set({ "n", "x", "o" }, "f", function() require("flash").jump() end, { desc = "Flash" })
vim.keymap.set({ "n", "x", "o" }, "F", function() require("flash").treesitter() end, { desc = "Flash Treesitter" })
vim.keymap.set("o", "r", function() require("flash").remote() end, { desc = "Remote Flash" })
vim.keymap.set({ "o", "x" }, "R", function() require("flash").treesitter_search() end, { desc = "Treesitter Search" })
vim.keymap.set("c", "<C-s>", function() require("flash").toggle() end, { desc = "Toggle Flash Search" })
