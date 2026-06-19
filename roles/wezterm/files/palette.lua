-- wezterm command palette — custom-only entries for augment-command-palette
-- Default palette already covers: new tab, close tab, splits, pane focus, activate tab.

local wezterm = require('wezterm')
local act = wezterm.action

local M = {}

local function cwd_string(pane)
  local cwd = pane:get_current_working_dir()
  if cwd and cwd.file_path then
    return cwd.file_path
  end
  return os.getenv('HOME')
end

local function folder_name(pane)
  local cwd = cwd_string(pane)
  return cwd:match('([^/]+)$') or cwd
end

local function wezterm_bin()
  local candidates = {
    '/Applications/WezTerm.app/Contents/MacOS/wezterm',
    '/opt/homebrew/bin/wezterm',
    'wezterm',
  }
  for _, path in ipairs(candidates) do
    if path:sub(1, 1) == '/' then
      local f = io.open(path, 'r')
      if f then
        f:close()
        return path
      end
    else
      return path
    end
  end
  return 'wezterm'
end

local function lpass_bin()
  local candidates = {
    '/opt/homebrew/bin/lpass',
    '/usr/local/bin/lpass',
    'lpass',
  }
  for _, path in ipairs(candidates) do
    if path:sub(1, 1) == '/' then
      local f = io.open(path, 'r')
      if f then
        f:close()
        return path
      end
    else
      return path
    end
  end
  return 'lpass'
end

local WEZTERM = wezterm_bin()
local LPASS = lpass_bin()

local function trim(s)
  return (s or ''):gsub('^%s+', ''):gsub('%s+$', '')
end

local function notify_lpass(msg)
  wezterm.run_child_process {
    'osascript',
    '-e',
    string.format('display notification %q with title "lpass"', msg),
  }
end

local function toast_err(win, msg)
  wezterm.log_error('lpass: ' .. msg)
  win:toast_notification('lpass', msg, 'Terminal', 5000)
  notify_lpass(msg)
end

local function run_lpass(args)
  local cmd = { LPASS }
  for _, a in ipairs(args) do
    table.insert(cmd, a)
  end
  local ok, stdout, stderr = wezterm.run_child_process(cmd)
  return ok, trim(stdout), trim(stderr)
end

local function fetch_lpass_entries()
  local ok, stdout, stderr = run_lpass { 'status' }
  if not ok then
    return nil, stderr ~= '' and stderr or stdout ~= '' and stdout or 'not logged in'
  end

  ok, stdout, stderr = run_lpass { 'ls', '--format', '%aN\t%ai' }
  if not ok then
    return nil, stderr ~= '' and stderr or stdout ~= '' and stdout or 'lpass ls failed'
  end

  local choices = {}
  for line in (stdout or ''):gmatch('[^\r\n]+') do
    local name, id = line:match('^(.-)\t(.+)$')
    if name and id and id ~= '' then
      table.insert(choices, { id = id, label = name })
    end
  end

  if #choices == 0 then
    return nil, 'no LastPass entries found'
  end
  return choices, nil
end

local function run_credential_action(win, action, entry_id)
  local args
  local done_msg
  if action == 'password' then
    args = { 'show', '--clip', '--password', entry_id }
    done_msg = 'password copied'
  elseif action == 'username' then
    args = { 'show', '--clip', '--username', entry_id }
    done_msg = 'username copied'
  elseif action == 'generate' then
    args = { 'generate', '--clip', entry_id, '20' }
    done_msg = 'password generated'
  else
    toast_err(win, 'unknown action: ' .. action)
    return
  end

  local ok, stdout, stderr = run_lpass(args)
  if not ok then
    toast_err(win, stderr ~= '' and stderr or stdout ~= '' and stdout or 'lpass failed')
    return
  end
  notify_lpass(done_msg)
end

local function lpass_credential_action(action, title)
  return wezterm.action_callback(function(win, pane)
    local choices, err = fetch_lpass_entries()
    if not choices then
      toast_err(win, err)
      return
    end
    win:perform_action(
      act.InputSelector {
        title = title,
        choices = choices,
        fuzzy = true,
        fuzzy_description = 'lpass ❯ ',
        action = wezterm.action_callback(function(w, _p, id)
          if id then
            run_credential_action(w, action, id)
          end
        end),
      },
      pane
    )
  end)
end

local function lpass_add_action()
  return wezterm.action_callback(function(win, pane)
    local ok, stdout, stderr = run_lpass { 'status' }
    if not ok then
      toast_err(win, stderr ~= '' and stderr or stdout ~= '' and stdout or 'not logged in')
      return
    end
    win:perform_action(
      act.PromptInputLine {
        description = 'entry name',
        action = wezterm.action_callback(function(w, _p, line)
          if not line or line == '' then
            return
          end
          local add_ok, add_out, add_err = run_lpass { 'add', line }
          if not add_ok then
            toast_err(w, add_err ~= '' and add_err or add_out ~= '' and add_out or 'lpass add failed')
            return
          end
          notify_lpass('added ' .. line)
        end),
      },
      pane
    )
  end)
end

local function kill_pane_id(pane_id)
  wezterm.run_child_process {
    WEZTERM,
    'cli',
    'kill-pane',
    '--pane-id',
    tostring(pane_id),
  }
end

local function entry(brief, icon, action, doc)
  return {
    brief = brief,
    icon = icon,
    doc = doc,
    action = action,
  }
end

local function prompt_rename_tab()
  return act.PromptInputLine {
    description = 'tab name',
    action = wezterm.action_callback(function(window, _pane, line)
      if line then
        window:active_tab():set_title(line)
      end
    end),
  }
end

local function first_pane_in_tab(tab)
  local panes = tab:panes()
  if #panes == 0 then
    return nil
  end
  return panes[1]
end

local function move_pane_to_tab_action(window, pane)
  local mux_win = window:mux_window()
  local current_tab_id = window:active_tab():tab_id()
  local source_pane_id = pane:pane_id()
  local choices = {}
  local tabs_by_id = {}

  for _, info in ipairs(mux_win:tabs_with_info()) do
    local tab = info.tab
    local tab_id = tab:tab_id()
    if tab_id ~= current_tab_id then
      local id = tostring(tab_id)
      table.insert(choices, {
        id = id,
        label = tab:get_title(),
      })
      tabs_by_id[id] = tab
    end
  end

  if #choices == 0 then
    return wezterm.action_callback(function(win)
      win:toast_notification('wezterm', 'No other tabs', 'Terminal', 3000)
    end)
  end

  return act.InputSelector {
    title = 'Move pane to tab',
    choices = choices,
    fuzzy = true,
    fuzzy_description = 'to tab ❯ ',
    action = wezterm.action_callback(function(win, _p, id)
      if not id then
        return
      end

      local tab = tabs_by_id[id]
      if not tab then
        return
      end

      local target_pane = first_pane_in_tab(tab)
      if not target_pane then
        win:toast_notification('wezterm', 'Target tab has no panes', 'Terminal', 4000)
        return
      end

      local ok, stdout, stderr = wezterm.run_child_process {
        WEZTERM,
        'cli',
        'split-pane',
        '--pane-id',
        tostring(target_pane:pane_id()),
        '--right',
        '--move-pane-id',
        tostring(source_pane_id),
      }
      if not ok then
        local err = stderr or stdout or 'unknown error'
        wezterm.log_error('move pane to tab failed: ' .. err)
        win:toast_notification('wezterm', 'Move pane failed: ' .. err, 'Terminal', 5000)
      end
    end),
  }
end

function M.build_entries(window, pane)
  local ok, entries = pcall(M._build_entries, window, pane)
  if not ok then
    wezterm.log_error('palette: ' .. tostring(entries))
    return {
      {
        brief = 'Palette: load error (see wezterm log)',
        icon = 'md_alert',
        action = wezterm.action_callback(function(win)
          win:toast_notification('wezterm', tostring(entries), 'Terminal', 8000)
        end),
      },
    }
  end
  return entries
end

function M._build_entries(window, pane)
  local mux_win = window:mux_window()
  local entries = {}

  local function add(brief, icon, action, doc)
    table.insert(entries, entry(brief, icon, action, doc))
  end

  -- Tab — custom only (defaults cover new/close/switch)
  add(
    'Tab: rename to current folder',
    'md_folder_rename',
    wezterm.action_callback(function(win, p)
      win:active_tab():set_title(folder_name(p))
    end)
  )
  add('Tab: rename current tab', 'md_rename_box', prompt_rename_tab())
  add(
    'Tab: close all others',
    'md_tab_unselected',
    wezterm.action_callback(function(_win, _pane)
      for _, info in ipairs(mux_win:tabs_with_info()) do
        if not info.is_active then
          for _, p in ipairs(info.tab:panes()) do
            kill_pane_id(p:pane_id())
          end
        end
      end
    end)
  )

  -- Pane — custom only (defaults cover split/focus/close via keybindings)
  add(
    'Pane: close all others',
    'md_select_remove',
    wezterm.action_callback(function(win, p)
      local current_id = p:pane_id()
      for _, other in ipairs(win:active_tab():panes()) do
        if other:pane_id() ~= current_id then
          kill_pane_id(other:pane_id())
        end
      end
    end)
  )
  add(
    'Pane: break out to new tab',
    'md_tab_unselected',
    wezterm.action_callback(function(_win, p)
      p:move_to_new_tab()
    end)
  )
  add('Pane: move to another tab', 'md_tab_move', move_pane_to_tab_action(window, pane))

  -- Credentials — lpass list loads on selection (not when palette opens)
  add('Credential: copy password', 'md_key', lpass_credential_action('password', 'Copy password'))
  add('Credential: copy username', 'md_account', lpass_credential_action('username', 'Copy username'))
  add('Credential: add new', 'md_key_plus', lpass_add_action())
  add('Credential: generate password', 'md_key_change', lpass_credential_action('generate', 'Generate password'))

  return entries
end

function M.register()
  -- Kept for backwards compatibility; wezterm.lua registers the event directly.
end

return M
