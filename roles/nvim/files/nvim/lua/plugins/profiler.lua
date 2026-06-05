return {
  -- Move the profiler scratch-buffer key from <leader>dp to the code group <leader>cp
  {
    "folke/snacks.nvim",
    keys = {
      { "<leader>dps", false },
      { "<leader>cps", function() Snacks.profiler.scratch() end, desc = "Profiler Scratch Buffer" },
    },
  },

  -- Re-home the profiler which-key group under <leader>c (code), hide the old <leader>dp one
  {
    "folke/which-key.nvim",
    opts = {
      spec = {
        { "<leader>dp", group = "profiler", hidden = true },
        { "<leader>cp", group = "profiler" },
      },
    },
  },
}
