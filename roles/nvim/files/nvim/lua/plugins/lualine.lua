return {
  {
    "nvim-lualine/lualine.nvim",
    opts = function(_, opts)
      local cwd_branch = ""
      local function refresh_cwd_branch()
        vim.system(
          { "git", "-C", vim.fn.getcwd(), "rev-parse", "--abbrev-ref", "HEAD" },
          { text = true },
          function(out)
            cwd_branch = out.code == 0 and vim.trim(out.stdout) or ""
          end
        )
      end
      refresh_cwd_branch()
      vim.api.nvim_create_autocmd({ "DirChanged", "FocusGained" }, {
        callback = refresh_cwd_branch,
      })

      opts.sections.lualine_b = {
        {
          function()
            return cwd_branch
          end,
          cond = function()
            return cwd_branch ~= ""
          end,
          icon = "\238\130\160",
        },
      }
      opts.sections.lualine_c = {}
      opts.sections.lualine_y = {
        {
          function()
            return vim.fn.fnamemodify(vim.fn.getcwd(), ":~")
          end,
        },
      }
      opts.sections.lualine_z = {}
      opts.winbar = { lualine_x = { "filename" } }
      opts.inactive_winbar = { lualine_x = { "filename" } }
      return opts
    end,
  },
}
