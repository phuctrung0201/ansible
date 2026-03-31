# Kitty Leader — Rust Implementation Plan

## Overview

Replace `roles/kitty/files/palette.sh` with a compiled Rust binary at `~/.config/kitty/leader`. A which-key-style TUI built with ratatui. No fzf.

---

## Project Structure

```
roles/kitty/
  files/
    kitty.conf           # add opt+l keybinding, keep opt+p palette.sh untouched
    kitty.zsh
    palette.sh           # unchanged — opt+p continues to work
  leader/
    Cargo.toml
    Makefile
    src/
      main.rs
      kitty.rs
      action.rs
      keymap.rs
      leader.rs
  tasks/
    main.yml             # add build + copy tasks for leader only
```

---

## Cargo.toml

```toml
[package]
name = "leader"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "leader"
path = "src/main.rs"

[dependencies]
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
anyhow      = "1"
ratatui     = "0.29"

[profile.release]
strip         = true
opt-level     = "z"
lto           = true
codegen-units = 1
```

---

## Makefile

```makefile
.PHONY: run build install

run:
	cargo run

build:
	cargo build --release

install: build
	cp target/release/leader ~/.config/kitty/leader
```

`make run` launches the TUI directly for testing — kitty remote calls will fail gracefully since there is no socket, but rendering and key navigation are fully exercisable.

---

## Modules

### `src/main.rs`

```rust
fn main() -> anyhow::Result<()> {
    leader::run()
}
```

---

### `src/kitty.rs`

Raw `kitten @` subprocess calls only. No structs, no logic. All functions prepend `/opt/homebrew/bin` to PATH.

```rust
pub fn ls() -> anyhow::Result<String>;
pub fn send_action(cmd: &str) -> anyhow::Result<()>;
pub fn send_text(code: &str) -> anyhow::Result<()>;
pub fn close_window_self() -> anyhow::Result<()>;
pub fn focus_tab(id: u64) -> anyhow::Result<()>;
pub fn close_tab(id: u64) -> anyhow::Result<()>;
pub fn detach_tab_self(target_tab_id: u64) -> anyhow::Result<()>;
```

---

### `src/action.rs`

JSON structs for `kitten @ ls`, named action functions, and leader state.

```rust
// JSON structs
#[derive(Deserialize)]
pub struct KittyOs { pub tabs: Vec<KittyTab> }

#[derive(Deserialize)]
pub struct KittyTab {
    pub id: u64,
    pub title: String,
    pub is_focused: bool,
    #[serde(default)]
    pub active_window_history: Vec<u64>,
    pub windows: Vec<KittyWindow>,
}

#[derive(Deserialize)]
pub struct KittyWindow { pub id: u64, pub is_self: bool }

fn parse_ls() -> anyhow::Result<Vec<KittyOs>>;

// Named action functions — each closes the overlay then executes
pub fn open_url()           -> anyhow::Result<()>;
pub fn copy_url()           -> anyhow::Result<()>;
pub fn copy_file_path()     -> anyhow::Result<()>;
pub fn edit_command()       -> anyhow::Result<()>;
pub fn search_history()     -> anyhow::Result<()>;
pub fn copy_last_output()   -> anyhow::Result<()>;
pub fn open_scrollback()    -> anyhow::Result<()>;
pub fn new_window()         -> anyhow::Result<()>;
pub fn new_window_here()    -> anyhow::Result<()>;
pub fn new_tab()            -> anyhow::Result<()>;
pub fn new_tab_here()       -> anyhow::Result<()>;
pub fn detach_tab()         -> anyhow::Result<()>;

// Custom actions — call leader::pick for secondary selection
pub fn tab_switch()         -> anyhow::Result<()>;
pub fn close_other_tabs()   -> anyhow::Result<()>;
pub fn move_tab_to_window() -> anyhow::Result<()>;

// Leader state — tracks the currently visible node slice
pub struct LeaderState<'a> {
    pub nodes: &'a [keymap::KeyNode],
}

pub enum KeyPress {
    Redraw,
    Execute(fn() -> anyhow::Result<()>),
    Unrecognised,
}

/// Match key against current nodes. Group match → update state, return Redraw.
/// Action match → return Execute. No match → return Unrecognised.
pub fn press_key(state: &mut LeaderState, key: char) -> KeyPress;
```

---

### `src/keymap.rs`

Key tree definition. Pure data — no logic. References `action::*` functions by pointer.

```rust
pub struct KeyNode {
    pub key: char,
    pub label: &'static str,
    pub kind: KeyNodeKind,
}

pub enum KeyNodeKind {
    Action(fn() -> anyhow::Result<()>),
    // icon: displayed in the popup header when this group is active
    Group { icon: &'static str, nodes: &'static [KeyNode] },
}

pub static KEYMAP: &[KeyNode] = &[
    KeyNode { key: 'h', label: "hints", kind: KeyNodeKind::Group { icon: "󰍉", nodes: &[
        KeyNode { key: 'u', label: "open url",       kind: KeyNodeKind::Action(action::open_url) },
        KeyNode { key: 'c', label: "copy url",       kind: KeyNodeKind::Action(action::copy_url) },
        KeyNode { key: 'f', label: "copy file path", kind: KeyNodeKind::Action(action::copy_file_path) },
    ]}},
    KeyNode { key: 't', label: "terminal", kind: KeyNodeKind::Group { icon: "", nodes: &[
        KeyNode { key: 'e', label: "edit command",   kind: KeyNodeKind::Action(action::edit_command) },
        KeyNode { key: 's', label: "search history", kind: KeyNodeKind::Action(action::search_history) },
        KeyNode { key: 'o', label: "copy last out",  kind: KeyNodeKind::Action(action::copy_last_output) },
        KeyNode { key: 'b', label: "scrollback",     kind: KeyNodeKind::Action(action::open_scrollback) },
    ]}},
    KeyNode { key: 'w', label: "window", kind: KeyNodeKind::Group { icon: "", nodes: &[
        KeyNode { key: 'n', label: "new",            kind: KeyNodeKind::Action(action::new_window) },
        KeyNode { key: 'h', label: "new from here",  kind: KeyNodeKind::Action(action::new_window_here) },
    ]}},
    KeyNode { key: 'b', label: "tabs", kind: KeyNodeKind::Group { icon: "󰓩", nodes: &[
        KeyNode { key: 'n', label: "new",            kind: KeyNodeKind::Action(action::new_tab) },
        KeyNode { key: 'h', label: "new from here",  kind: KeyNodeKind::Action(action::new_tab_here) },
        KeyNode { key: 's', label: "switch",         kind: KeyNodeKind::Action(action::tab_switch) },
        KeyNode { key: 'c', label: "close others",   kind: KeyNodeKind::Action(action::close_other_tabs) },
        KeyNode { key: 'd', label: "detach",         kind: KeyNodeKind::Action(action::detach_tab) },
        KeyNode { key: 'm', label: "move to window", kind: KeyNodeKind::Action(action::move_tab_to_window) },
    ]}},
];
```

---

### `src/leader.rs`

Ratatui event loop, `render`, and `pick`. Ratatui manages terminal setup, alternate screen, raw mode, and double-buffered rendering.

```rust
pub fn run() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = LeaderState { nodes: keymap::KEYMAP };
    let result = event_loop(&mut terminal, &mut state);
    ratatui::restore();
    result
}

fn event_loop(terminal: &mut Terminal<impl Backend>, state: &mut LeaderState) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        match event::read()? {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => return Ok(()),
            Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) => {
                match action::press_key(state, c) {
                    KeyPress::Execute(f)  => return f(),
                    KeyPress::Redraw | KeyPress::Unrecognised => {}
                }
            }
            _ => {}
        }
    }
}
```

**`render`** — draws a which-key hints panel centered horizontally and anchored to the bottom of the frame. Layout is computed with two passes:
1. Vertical `Layout` splits the frame into `[Fill, Fixed(popup_height)]` — popup occupies the last N rows
2. Horizontal `Layout` on the bottom area splits into `[Fill, Fixed(popup_width), Fill]` — popup is centered

`popup_width` = `min(terminal_width, 72)`. `popup_height` = `ceil(items / 4) + 2` (border rows included).

Ratatui `Block` (rounded border) + `Paragraph` of `Span`s. Dracula colors:
- Border: purple `#bd93f9`
- Key badge: cyan bg `#8be9fd`, dark fg `#282a36`
- Label: white `#f8f8f2`
- Group header: comment grey `#6272a4`

The popup header is `"{icon} {label}"` taken from the active `KeyNode`:
- Root level: hardcoded `"⚡ leader"`
- Inside a group: the matched `KeyNode`'s `icon` + `label` from `KeyNodeKind::Group`

```
╭───────────────────── ⚡ leader ─────────────────────╮
│  [h] hints    [t] terminal   [w] window   [b] tabs   │
╰─────────────────────────────────────────────────────╯

╭────────────────────  terminal ─────────────────╮
│  [e] edit cmd  [s] search  [o] last out  [b] scrollback │
╰─────────────────────────────────────────────────────────╯
```

**`pick`** — secondary selection for `tab_switch` and `move_tab_to_window`. Same hints-panel style: keys `a`–`z` auto-assigned to items, single keypress selects, Esc returns `None`. Items are grouped with styled section headers — no scroll, no cursor.

```rust
pub struct PickGroup<'a> {
    pub label: &'a str,
    pub items: &'a [&'a str],
}

pub fn pick(prompt: &str, groups: &[PickGroup]) -> anyhow::Result<Option<(usize, usize)>>;
```

```
╭──────────── 📑 switch tab ─────────────╮
│  window 1                              │
│  [a] term: nvim    [b] term: zsh       │
│  window 2                              │
│  [c] browser                           │
╰────────────────────────────────────────╯
```

---

## Ansible Integration

### `roles/kitty/tasks/main.yml`

```yaml
- name: Build leader binary
  command:
    cmd: cargo build --release --manifest-path roles/kitty/leader/Cargo.toml
    chdir: "{{ playbook_dir }}"
  changed_when: true

- name: Copy leader binary
  copy:
    src: "{{ playbook_dir }}/roles/kitty/leader/target/release/leader"
    dest: "{{ ansible_env.HOME }}/.config/kitty/leader"
    mode: "0755"
```

### `roles/kitty/files/kitty.conf`

Add alongside the existing `opt+p` binding — do not remove or modify it.

```
map opt+p launch --cwd=current --type=overlay ~/.config/kitty/palette.sh  # existing
map opt+l launch --cwd=current --type=overlay ~/.config/kitty/leader      # new
```

### `roles/mise/files/config`

```toml
[tools]
rust = "1.91"
```

---

## Error Handling

`anyhow::Result` throughout. `.context("...")` at every `?` site. `anyhow::bail!` for assertion errors. Errors surface on stderr, visible in the kitty overlay before it closes.

---

## Implementation Sequence

1. `src/kitty.rs` — raw kitten primitives
2. `src/action.rs` — JSON structs, named action fns, `LeaderState`, `press_key`
3. `src/keymap.rs` — `KeyNode`, `KeyNodeKind`, `KEYMAP`
4. `src/leader.rs` — event loop, `render`, `pick`
5. `src/main.rs` — call `leader::run()`
6. `Makefile` — `run`, `build`, `install`
7. `roles/kitty/tasks/main.yml` — build + copy tasks
8. `roles/kitty/files/kitty.conf` — `opt+l` keybinding
9. `roles/mise/files/config` — `rust = "1.91"`
