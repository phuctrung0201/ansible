use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

use crate::{action::{LeaderState, KeyPress}, keymap};

// ---------------------------------------------------------------------------
// Dracula palette (https://spec.draculatheme.com/)
// ---------------------------------------------------------------------------
const MAUVE: Color = Color::Rgb(189, 147, 249);   // purple
const ORANGE: Color = Color::Rgb(255, 184, 108);  // #ffb86c — git branch pill
const TEAL: Color = Color::Rgb(139, 233, 253);    // cyan
const GREEN: Color = Color::Rgb(80, 250, 123);    // #50fa7b — cwd pill
const YELLOW: Color = Color::Rgb(241, 250, 140);  // yellow
const FG: Color = Color::Rgb(248, 248, 242);      // foreground
const COMMENT: Color = Color::Rgb(98, 114, 164); // comment
const DRACULA_BG: Color = Color::Rgb(40, 42, 54);  // #282a36
const PILL_BG: Color = Color::Rgb(68, 71, 90);   // #44475a (selection / inactive window pills)
/// Dracula Cyan (`#8BE9FD`) — kube context pill fill; cwd pill uses `GREEN` in `cwd_pill_spans`.
const CWD_PILL_BG: Color = TEAL;

const COLS: usize = 4;
const KEY_WIDTH: usize = 5; // widest key label is "space" (5 chars)

/// Nerd Fonts / Powerline Extra. Both caps use `bg(DRACULA_BG)` / `fg` = pill fill.
/// Left/right glyphs swapped vs airline defaults so curves match this layout.
const ROUND_CAP_L: &str = "\u{e0b6}";
const ROUND_CAP_R: &str = "\u{e0b4}";

/// Font Awesome `bolt` — `─── actions ───` title (same glyph as `LEADER_HEADER_ICON` in `action.rs`).
const ACTIONS_TITLE_ICON: &str = "\u{f0e7}";

/// Font Awesome `window-maximize` (same PUA style as `LEADER_HEADER_ICON` in `action.rs`).
const WINDOWS_SECTION_ICON: &str = "\u{f2d0}";

/// Matches tab group icon in `keymap` (`󰓩`).
const TABS_SECTION_ICON: &str = "󰓩";

/// Matches launcher group icon in `keymap` (`󱓞`).
const LAUNCHER_SECTION_ICON: &str = "󱓞";

/// Font Awesome `folder-open` (PUA); pair with Nerd Fonts / merged FA like other leader icons.
const CWD_PILL_ICON: &str = "\u{f07c}";

/// Nerd Fonts Devicons `nf-dev-kubernetes` (`U+E81D`). Use a patched Nerd Font in kitty.
const KUBE_PILL_ICON: &str = "\u{e81d}";

/// Nerd Fonts Devicons `nf-dev-git_branch` (`U+E725`).
const GIT_PILL_ICON: &str = "\u{e725}";

// ---------------------------------------------------------------------------
// PickGroup — secondary selection
// ---------------------------------------------------------------------------

pub struct PickItem {
    pub label: String,
    pub focused: bool,
    pub current: bool,
}

pub struct PickGroup {
    pub label: String,
    pub items: Vec<PickItem>,
}

// ---------------------------------------------------------------------------
// Shared slot rendering helpers
// ---------------------------------------------------------------------------

fn key_display(key: char) -> String {
    let s = match key {
        ' ' => "space".to_string(),
        '\t' => "tab".to_string(),
        _ => key.to_string(),
    };
    format!("{:>KEY_WIDTH$}", s)
}

/// Returns the label column width given a popup's inner width.
fn label_width(inner_width: usize) -> usize {
    let slot_width = inner_width / COLS;
    slot_width.saturating_sub(KEY_WIDTH + 3 + 6) // badge(KEY_WIDTH) + " → "(3) + trailing(6)
}

fn slot_spans_str(key: &str, label: &str, icon: &str, lw: usize, is_last: bool, focused: bool, current: bool) -> [Span<'static>; 2] {
    let trailing = if is_last { 0 } else { 6 };
    let icon_chars = icon.chars().count();
    let max_label = lw.saturating_sub(icon_chars);
    let label: std::borrow::Cow<str> = if label.chars().count() > max_label {
        label.chars().take(max_label.saturating_sub(1)).chain(std::iter::once('…')).collect::<String>().into()
    } else {
        label.into()
    };
    let text = format!(
        " → {}{:<width$}{:>trail$}",
        icon,
        label,
        "",
        width = max_label,
        trail = trailing,
    );
    let label_style = if focused {
        Style::default().fg(MAUVE).add_modifier(Modifier::BOLD)
    } else if current {
        Style::default().fg(YELLOW).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(FG)
    };
    let key_str = format!("{:>KEY_WIDTH$}", key);
    let key_style = if current {
        Style::default().fg(YELLOW).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TEAL).add_modifier(Modifier::BOLD)
    };
    [
        Span::styled(key_str, key_style),
        Span::styled(text, label_style),
    ]
}

/// Two spans for a single key-badge + label slot.
fn slot_spans(key: char, label: &str, icon: &str, lw: usize, is_last: bool, focused: bool) -> [Span<'static>; 2] {
    slot_spans_str(&key_display(key), label, icon, lw, is_last, focused, false)
}

fn top_rect(width: u16, height: u16, area: Rect) -> Rect {
    Rect { x: area.x, y: area.y, width: width.min(area.width), height: height.min(area.height) }
}

fn popup_block() -> Block<'static> {
    Block::new()
        .style(Style::default().bg(DRACULA_BG))
        .padding(ratatui::widgets::Padding::new(2, 2, 1, 0))
}

fn truncate_pill_label(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        s.chars()
            .take(max_chars.saturating_sub(1))
            .chain(std::iter::once('…'))
            .collect()
    }
}

fn pill_style(selected: bool, kitty_focused: bool, recent: bool) -> Style {
    if selected {
        Style::default()
            .fg(DRACULA_BG)
            .bg(MAUVE)
            .add_modifier(Modifier::BOLD)
    } else if kitty_focused {
        Style::default()
            .fg(MAUVE)
            .bg(PILL_BG)
            .add_modifier(Modifier::BOLD)
    } else if recent {
        Style::default()
            .fg(DRACULA_BG)
            .bg(YELLOW)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(FG).bg(PILL_BG)
    }
}

/// Pill fill color for caps (must match middle `bg` for non-selected states).
fn pill_cap_fill_color(selected: bool, current: bool) -> Color {
    if selected {
        MAUVE
    } else if current {
        YELLOW
    } else {
        PILL_BG
    }
}

fn popup_gap() -> Span<'static> {
    Span::styled(" ", Style::default().bg(DRACULA_BG))
}

fn rule_span(s: String) -> Span<'static> {
    Span::styled(s, Style::default().fg(COMMENT).bg(DRACULA_BG))
}

/// Blank row used as vertical margin around section dividers.
fn section_spacer_line() -> Line<'static> {
    Line::from("")
}

/// Horizontal rule with centered label, e.g. `───  windows  ───` (width = display columns).
fn titled_rule_line(title: &str, width: usize) -> Line<'static> {
    let label = format!("  {}  ", title);
    let lw = label.chars().count();
    if width <= lw.saturating_add(2) {
        return Line::from(vec![Span::styled(
            label,
            Style::default()
                .fg(MAUVE)
                .bg(DRACULA_BG)
                .add_modifier(Modifier::BOLD),
        )])
        .alignment(Alignment::Center);
    }
    let rules = width.saturating_sub(lw);
    let left_len = rules / 2;
    let right_len = rules - left_len;
    Line::from(vec![
        rule_span("─".repeat(left_len)),
        Span::styled(
            label,
            Style::default()
                .fg(MAUVE)
                .bg(DRACULA_BG)
                .add_modifier(Modifier::BOLD),
        ),
        rule_span("─".repeat(right_len)),
    ])
    .alignment(Alignment::Center)
}

/// Top spacer, horizontal rule, bottom spacer.
fn divider_with_vertical_margin(title: &str, width: usize) -> Vec<Line<'static>> {
    vec![
        section_spacer_line(),
        titled_rule_line(title, width),
        section_spacer_line(),
    ]
}

fn cwd_pill_spans(cwd: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = CWD_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(cwd, max_text);
    let inner = format!(" {} {} ", CWD_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(DRACULA_BG)
        .bg(GREEN)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(GREEN).bg(DRACULA_BG);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

fn kube_pill_spans(ctx: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = KUBE_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(ctx, max_text);
    let inner = format!(" {} {} ", KUBE_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(DRACULA_BG)
        .bg(CWD_PILL_BG)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(CWD_PILL_BG).bg(DRACULA_BG);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

fn git_pill_spans(branch: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = GIT_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(branch, max_text);
    let inner = format!(" {} {} ", GIT_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(DRACULA_BG)
        .bg(ORANGE)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(ORANGE).bg(DRACULA_BG);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

/// Cwd / kube / git spans for the top row (no alignment). Used standalone (centered) or prefixed to window/tab/launcher pills.
fn banner_pills_prefix_spans(
    cwd: Option<&str>,
    kube: Option<&str>,
    git: Option<&str>,
    max_line_width: usize,
) -> Option<Vec<Span<'static>>> {
    let n = usize::from(cwd.is_some())
        + usize::from(kube.is_some())
        + usize::from(git.is_some());
    if n == 0 {
        return None;
    }
    if n == 1 {
        let spans = match (cwd, kube, git) {
            (Some(c), None, None) => cwd_pill_spans(c, max_line_width),
            (None, Some(k), None) => kube_pill_spans(k, max_line_width),
            (None, None, Some(g)) => git_pill_spans(g, max_line_width),
            _ => return None,
        };
        return Some(spans);
    }
    let gaps = n - 1;
    let w = (max_line_width.saturating_sub(gaps) / n).max(18);
    let mut spans: Vec<Span<'static>> = Vec::new();
    if let Some(c) = cwd {
        if !spans.is_empty() {
            spans.push(popup_gap());
        }
        spans.extend(cwd_pill_spans(c, w));
    }
    if let Some(k) = kube {
        if !spans.is_empty() {
            spans.push(popup_gap());
        }
        spans.extend(kube_pill_spans(k, w));
    }
    if let Some(g) = git {
        if !spans.is_empty() {
            spans.push(popup_gap());
        }
        spans.extend(git_pill_spans(g, w));
    }
    Some(spans)
}

/// Centered top row: cwd / kube / git status pills (read‑only), above windows/tabs/launcher lists.
fn banner_pills_line(
    cwd: Option<&str>,
    kube: Option<&str>,
    git: Option<&str>,
    max_line_width: usize,
) -> Option<Line<'static>> {
    let spans = banner_pills_prefix_spans(cwd, kube, git, max_line_width)?;
    Some(Line::from(spans).alignment(Alignment::Center))
}

/// Wrapped horizontal lines of window “pills” for the root header.
fn window_pill_lines(
    rows: &[crate::action::LeaderWindowRow],
    cursor: usize,
    max_line_width: usize,
) -> Vec<Line<'static>> {
    const MAX_WINDOWS: usize = 24;
    const MIN_CHARS: usize = 6;
    let max_chars = (max_line_width / 5).clamp(MIN_CHARS, 22);

    let mut out: Vec<Line<'static>> = Vec::new();
    let mut line_spans: Vec<Span<'static>> = Vec::new();
    let mut used = 0usize;

    for (i, row) in rows.iter().enumerate().take(MAX_WINDOWS) {
        let label = truncate_pill_label(&row.label, max_chars);
        let inner: String = if i < 9 {
            format!(" {} {} ", i + 1, label)
        } else {
            format!(" {} ", label)
        };
        let sel = i == cursor;
        let bg = pill_cap_fill_color(sel, row.current);
        let w = inner.chars().count() + 2; // rounded caps
        let gap = if line_spans.is_empty() { 0 } else { 1 };

        if used + gap + w > max_line_width && !line_spans.is_empty() {
            out.push(Line::from(line_spans));
            line_spans = Vec::new();
            used = 0;
        }
        if !line_spans.is_empty() {
            line_spans.push(popup_gap());
            used += 1;
        }
        let mid = pill_style(sel, row.focused, row.current);
        let cap_style = Style::default().fg(bg).bg(DRACULA_BG);
        line_spans.push(Span::styled(ROUND_CAP_L, cap_style));
        line_spans.push(Span::styled(inner, mid));
        line_spans.push(Span::styled(ROUND_CAP_R, cap_style));
        used += w;
    }

    if !line_spans.is_empty() {
        out.push(Line::from(line_spans));
    }
    if rows.len() > MAX_WINDOWS {
        out.push(Line::from(vec![Span::styled(
            format!("… +{} more", rows.len() - MAX_WINDOWS),
            Style::default().fg(COMMENT).bg(DRACULA_BG),
        )]));
    }
    out
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn show_message(title: &str, body: &str) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let result = message_loop(&mut terminal, title, body);
    ratatui::restore();
    result
}

fn message_loop(
    terminal: &mut Terminal<impl Backend>,
    title: &str,
    body: &str,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render_message(frame, title, body))?;
        if let Event::Key(KeyEvent { kind: KeyEventKind::Press, .. }) = event::read()? {
            return Ok(());
        }
    }
}

fn render_message(frame: &mut Frame, title: &str, body: &str) {
    let area = frame.area();
    let div_w = (area.width as usize).saturating_sub(8).max(8);
    let block = popup_block();
    let mut lines = divider_with_vertical_margin(title, div_w);
    lines.push(Line::from(body.to_owned()));
    frame.render_widget(
        Paragraph::new(lines)
            .style(Style::default().fg(FG).bg(DRACULA_BG))
            .centered()
            .block(block),
        area,
    );
}

pub fn run() -> anyhow::Result<()> {
    let os = crate::action::parse_ls().unwrap_or_default();
    if crate::action::should_skip_duplicate_leader_launch_from_os(&os) {
        crate::kitty::close_window_self()?;
        return Ok(());
    }
    let mut terminal = ratatui::init();
    let mut state = LeaderState::from_kitty_ls(os);
    let result = event_loop(&mut terminal, &mut state);
    ratatui::restore();
    result
}

// ---------------------------------------------------------------------------
// Event loop
// ---------------------------------------------------------------------------

fn is_root(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::KEYMAP.as_ptr())
}

fn is_tab_group(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::TAB_GROUP_NODES.as_ptr())
}

fn is_launch_group(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::LAUNCH_GROUP_NODES.as_ptr())
}

fn event_loop(
    terminal: &mut Terminal<impl Backend>,
    state: &mut LeaderState,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                ..
            }) => return Ok(()),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if is_root(state) && !state.window_rows.is_empty() {
                    let id = state.window_rows[state.window_cursor].id;
                    ratatui::restore();
                    return crate::action::focus_window_from_leader(id);
                }
                if is_tab_group(state) && !state.tab_rows.is_empty() {
                    let id = state.tab_rows[state.tab_cursor].id;
                    ratatui::restore();
                    return crate::action::focus_tab_from_leader(id);
                }
                if is_launch_group(state) && !state.launch_rows.is_empty() {
                    let idx = state.launch_rows[state.launch_cursor].id as usize;
                    ratatui::restore();
                    return crate::action::execute_launch_at(idx);
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if is_root(state) && !state.window_rows.is_empty() {
                    state.window_cursor = (state.window_cursor + 1) % state.window_rows.len();
                    continue;
                }
                if is_tab_group(state) && !state.tab_rows.is_empty() {
                    state.tab_cursor = (state.tab_cursor + 1) % state.tab_rows.len();
                    continue;
                }
                if is_launch_group(state) && !state.launch_rows.is_empty() {
                    state.launch_cursor =
                        (state.launch_cursor + 1) % state.launch_rows.len();
                    continue;
                }
                match crate::action::press_key(state, '\t') {
                    KeyPress::Execute(f) => {
                        ratatui::restore();
                        return f();
                    }
                    KeyPress::Redraw | KeyPress::Unrecognised => {}
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if is_root(state) && !state.window_rows.is_empty() {
                    let len = state.window_rows.len();
                    state.window_cursor = (state.window_cursor + len - 1) % len;
                    continue;
                }
                if is_tab_group(state) && !state.tab_rows.is_empty() {
                    let len = state.tab_rows.len();
                    state.tab_cursor = (state.tab_cursor + len - 1) % len;
                    continue;
                }
                if is_launch_group(state) && !state.launch_rows.is_empty() {
                    let len = state.launch_rows.len();
                    state.launch_cursor = (state.launch_cursor + len - 1) % len;
                    continue;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            }) => {
                if is_root(state) && !state.window_rows.is_empty() {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let j = idx - 1;
                            if j < state.window_rows.len() {
                                let id = state.window_rows[j].id;
                                ratatui::restore();
                                return crate::action::focus_window_from_leader(id);
                            }
                        }
                    }
                }
                if is_tab_group(state) && !state.tab_rows.is_empty() {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let j = idx - 1;
                            if j < state.tab_rows.len() {
                                let id = state.tab_rows[j].id;
                                ratatui::restore();
                                return crate::action::focus_tab_from_leader(id);
                            }
                        }
                    }
                }
                if is_launch_group(state) && !state.launch_rows.is_empty() {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let j = idx - 1;
                            if j < state.launch_rows.len() {
                                let li = state.launch_rows[j].id as usize;
                                ratatui::restore();
                                return crate::action::execute_launch_at(li);
                            }
                        }
                    }
                }
                match crate::action::press_key(state, c) {
                    KeyPress::Execute(f) => {
                        ratatui::restore();
                        return f();
                    }
                    KeyPress::Redraw | KeyPress::Unrecognised => {}
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// render — which-key hints panel anchored to bottom of frame
// ---------------------------------------------------------------------------

fn render(frame: &mut Frame, state: &LeaderState) {
    let nodes = state.nodes;
    let area = frame.area();

    let block = popup_block();
    let div_w = (area.width as usize).saturating_sub(8).max(8);
    let header = format!("{} actions", ACTIONS_TITLE_ICON);

    let n_rows = if is_launch_group(state) {
        0
    } else {
        (nodes.len() as u16).div_ceil(COLS as u16)
    };

    let pill_max_w = (area.width as usize).saturating_sub(4).max(20);
    let has_any_banner = state.cwd_pill.is_some()
        || state.kube_pill.is_some()
        || state.git_pill.is_some();

    let mut top_strip: Vec<Line<'static>> = Vec::new();
    if is_root(state) && !state.window_rows.is_empty() {
        top_strip.extend(divider_with_vertical_margin(
            &format!("{} windows", WINDOWS_SECTION_ICON),
            div_w,
        ));
        top_strip.extend(window_pill_lines(
            &state.window_rows,
            state.window_cursor,
            pill_max_w,
        ));
    } else if is_tab_group(state) && !state.tab_rows.is_empty() {
        top_strip.extend(divider_with_vertical_margin(
            &format!("{} tabs", TABS_SECTION_ICON),
            div_w,
        ));
        top_strip.extend(window_pill_lines(
            &state.tab_rows,
            state.tab_cursor,
            pill_max_w,
        ));
    } else if is_launch_group(state) && !state.launch_rows.is_empty() {
        top_strip.extend(divider_with_vertical_margin(
            &format!("{} launcher", LAUNCHER_SECTION_ICON),
            div_w,
        ));
        top_strip.extend(window_pill_lines(
            &state.launch_rows,
            state.launch_cursor,
            pill_max_w,
        ));
    }
    let banner_lines = u16::from(has_any_banner);
    let strip_extra = banner_lines + top_strip.len() as u16;

    // Launcher: only the pill strip (no second titled rule, no key grid).
    // Actions divider: spacer + rule + spacer (see `divider_with_vertical_margin`).
    let header_rule_lines: u16 = if is_launch_group(state) { 0 } else { 3 };
    let popup_height = n_rows + strip_extra + header_rule_lines + 1;
    let popup_area = top_rect(area.width, popup_height, area);

    let inner_width = popup_area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let mut lines: Vec<Line> = Vec::new();
    if let Some(line) = banner_pills_line(
        state.cwd_pill.as_deref(),
        state.kube_pill.as_deref(),
        state.git_pill.as_deref(),
        pill_max_w,
    ) {
        lines.push(line);
    }
    lines.extend(top_strip);
    if !is_launch_group(state) {
        lines.extend(divider_with_vertical_margin(&header, div_w));
        for chunk in nodes.chunks(COLS) {
            let mut spans: Vec<Span> = Vec::new();
            for (i, node) in chunk.iter().enumerate() {
                let is_last = i + 1 == chunk.len();
                let icon = match &node.kind {
                    keymap::KeyNodeKind::Group { icon, .. } if !icon.is_empty() => {
                        format!("{} ", icon)
                    }
                    _ => String::new(),
                };
                let label = if matches!(&node.kind, keymap::KeyNodeKind::Group { .. }) {
                    format!("{}+", node.label)
                } else {
                    node.label.to_string()
                };
                spans.extend(slot_spans(node.key, &label, &icon, lw, is_last, false));
            }
            lines.push(Line::from(spans));
        }
    }

    frame.render_widget(
        Paragraph::new(lines)
            .style(Style::default().bg(DRACULA_BG))
            .block(block),
        popup_area,
    );
}

// ---------------------------------------------------------------------------
// pick — secondary selection for tab_switch / move_tab_to_window
// ---------------------------------------------------------------------------

pub fn pick(
    prompt: &str,
    groups: &[PickGroup],
) -> anyhow::Result<Option<(usize, usize)>> {
    let mut key_map: Vec<(usize, usize)> = Vec::new();
    for (gi, group) in groups.iter().enumerate() {
        for ii in 0..group.items.len() {
            key_map.push((gi, ii));
        }
    }

    let mut terminal = ratatui::init();
    let result = pick_loop(&mut terminal, prompt, groups, &key_map);
    ratatui::restore();
    result
}

fn pick_loop(
    terminal: &mut Terminal<impl Backend>,
    prompt: &str,
    groups: &[PickGroup],
    key_map: &[(usize, usize)],
) -> anyhow::Result<Option<(usize, usize)>> {
    let initial_cursor = key_map
        .iter()
        .position(|&(gi, ii)| groups[gi].items[ii].focused)
        .unwrap_or(0);
    let mut cursor = initial_cursor;

    loop {
        terminal.draw(|frame| render_pick(frame, prompt, groups, key_map, cursor))?;
        match event::read()? {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => return Ok(None),
            Event::Key(KeyEvent { code: KeyCode::Enter, kind: KeyEventKind::Press, .. }) => {
                if let Some(&pos) = key_map.get(cursor) {
                    return Ok(Some(pos));
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Tab, kind: KeyEventKind::Press, .. }) => {
                if !key_map.is_empty() {
                    cursor = (cursor + 1) % key_map.len();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if !key_map.is_empty() {
                    let len = key_map.len();
                    cursor = (cursor + len - 1) % len;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            }) => {
                if c.is_ascii_digit() && c != '0' {
                    let idx = (c as u8 - b'1') as usize;
                    if let Some(&(gi, ii)) = key_map.get(idx) {
                        return Ok(Some((gi, ii)));
                    }
                }
            }
            _ => {}
        }
    }
}

fn render_pick(
    frame: &mut Frame,
    prompt: &str,
    groups: &[PickGroup],
    key_map: &[(usize, usize)],
    cursor: usize,
) {
    let area = frame.area();
    let block = popup_block();
    let div_w = (area.width as usize).saturating_sub(8).max(8);

    let inner_width = area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let key_chars: std::collections::HashMap<(usize, usize), char> = key_map
        .iter()
        .enumerate()
        .filter_map(|(i, &(gi, ii))| {
            if i < 9 { Some(((gi, ii), (b'1' + i as u8) as char)) } else { None }
        })
        .collect();

    let cursor_pos = key_map.get(cursor).copied();

    let mut lines = divider_with_vertical_margin(prompt, div_w);

    for (gi, group) in groups.iter().enumerate() {
        if !group.label.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                group.label.clone(),
                Style::default().fg(COMMENT).bg(DRACULA_BG),
            )]));
        }

        let indices: Vec<usize> = (0..group.items.len()).collect();
        for chunk in indices.chunks(COLS) {
            let mut spans: Vec<Span> = Vec::new();
            for (ci, &ii) in chunk.iter().enumerate() {
                let is_last = ci + 1 == chunk.len();
                let key_char = key_chars.get(&(gi, ii)).copied().unwrap_or('?');
                let item = &group.items[ii];
                let focused = cursor_pos == Some((gi, ii));
                let key_str = key_char.to_string();
                let pair = slot_spans_str(&key_str, &item.label, "", lw, is_last, focused, item.current);
                spans.extend(pair);
            }
            lines.push(Line::from(spans));
        }
    }

    let content_rows = lines.len() as u16;
    let popup_height = content_rows.saturating_add(2).min(area.height);
    let list_area = top_rect(area.width, popup_height, area);

    frame.render_widget(
        Paragraph::new(lines)
            .style(Style::default().bg(DRACULA_BG))
            .block(block),
        list_area,
    );
}
