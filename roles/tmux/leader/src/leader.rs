use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    Frame, Terminal,
};

use crate::{action::{LeaderState, KeyPress}, keymap};

// ---------------------------------------------------------------------------
// Theme (loaded from ~/.config/tmux/leader-theme at runtime)
// ---------------------------------------------------------------------------

struct Theme {
    purple:    Color,
    cyan:      Color,
    yellow:    Color,
    fg:        Color,
    comment:   Color,
}

fn parse_hex(s: &str) -> Option<Color> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 { return None; }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

fn load_theme() -> Theme {
    let mut purple    = Color::Rgb(189, 147, 249);
    let mut cyan      = Color::Rgb(139, 233, 253);
    let mut yellow    = Color::Rgb(241, 250, 140);
    let mut fg        = Color::Rgb(248, 248, 242);
    let mut comment   = Color::Rgb(98,  114, 164);


    if let Ok(home) = std::env::var("HOME") {
        let path = format!("{home}/.config/tmux/leader-theme");
        if let Ok(contents) = std::fs::read_to_string(&path) {
            for line in contents.lines() {
                if let Some((key, val)) = line.split_once('=') {
                    if let Some(color) = parse_hex(val) {
                        match key.trim() {
                            "purple"    => purple    = color,
                            "cyan"      => cyan      = color,
                            "yellow"    => yellow    = color,
                            "fg"        => fg        = color,
                            "comment"   => comment   = color,

                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Theme { purple, cyan, yellow, fg, comment }
}

use std::sync::OnceLock;
static THEME: OnceLock<Theme> = OnceLock::new();
fn theme() -> &'static Theme {
    THEME.get_or_init(load_theme)
}

fn mauve()     -> Color { theme().purple }
fn teal()      -> Color { theme().cyan }
fn yellow()    -> Color { theme().yellow }
fn fg()        -> Color { theme().fg }
fn comment()   -> Color { theme().comment }

pub const COLS: usize = 4;


const KEY_WIDTH: usize = 5;

// ---------------------------------------------------------------------------
// PickGroup / PickItem
// ---------------------------------------------------------------------------

pub struct PickItem {
    pub label: String,
    pub current: bool,
}

pub struct PickGroup {
    pub label: String,
    pub items: Vec<PickItem>,
    pub initial_cursor: usize,
}

// ---------------------------------------------------------------------------
// Rendering helpers
// ---------------------------------------------------------------------------

fn key_display(key: char) -> String {
    let s = match key {
        ' '  => "space".to_string(),
        '\t' => "tab".to_string(),
        _    => key.to_string(),
    };
    format!("{:>KEY_WIDTH$}", s)
}

fn label_width(inner_width: usize) -> usize {
    let slot_width = inner_width / COLS;
    slot_width.saturating_sub(KEY_WIDTH + 3 + 4)
}

fn slot_spans_str(
    key: &str,
    label: &str,
    icon: &str,
    lw: usize,
    is_last: bool,
    focused: bool,
    current: bool,
) -> [Span<'static>; 2] {
    let trailing = if is_last { 0 } else { 4 };
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
        Style::default().fg(mauve()).add_modifier(Modifier::BOLD)
    } else if current {
        Style::default().fg(yellow())
    } else {
        Style::default().fg(fg())
    };
    let key_str = format!("{:>KEY_WIDTH$}", key);
    let key_style = if current {
        Style::default().fg(yellow()).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(teal()).add_modifier(Modifier::BOLD)
    };
    [
        Span::styled(key_str, key_style),
        Span::styled(text, label_style),
    ]
}

fn slot_spans(key: char, label: &str, icon: &str, lw: usize, is_last: bool, focused: bool) -> [Span<'static>; 2] {
    slot_spans_str(&key_display(key), label, icon, lw, is_last, focused, false)
}

fn top_rect(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.x,
        y: area.y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

fn popup_block(title: String) -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(mauve()))
        .padding(ratatui::widgets::Padding::new(2, 2, 1, 0))
        .title_top(
            Line::from(Span::styled(title, Style::default().fg(mauve()).add_modifier(Modifier::BOLD))).centered(),
        )
}

// ---------------------------------------------------------------------------
// Message overlay
// ---------------------------------------------------------------------------

pub fn show_message(title: &str, body: &str) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let result = message_loop(&mut terminal, title, body);
    ratatui::restore();
    result
}

fn message_loop(terminal: &mut Terminal<impl Backend>, title: &str, body: &str) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render_message(frame, title, body))?;
        if let Event::Key(KeyEvent { kind: KeyEventKind::Press, .. }) = event::read()? {
            return Ok(());
        }
    }
}

fn render_message(frame: &mut Frame, title: &str, body: &str) {
    let area = frame.area();
    let block = popup_block(format!(" {} ", title));
    frame.render_widget(
        Paragraph::new(body.to_owned()).centered().block(block),
        area,
    );
}

// ---------------------------------------------------------------------------
// Main which-key event loop
// ---------------------------------------------------------------------------

pub fn run() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = LeaderState::new();
    let result = event_loop(&mut terminal, &mut state);
    ratatui::restore();
    result
}

fn event_loop(terminal: &mut Terminal<impl Backend>, state: &mut LeaderState) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        match event::read()? {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => return Ok(()),
            Event::Key(KeyEvent { code: KeyCode::Char(c), kind: KeyEventKind::Press, .. }) => {
                match crate::action::press_key(state, c) {
                    KeyPress::Execute(f) => {
                        ratatui::restore();
                        return f();
                    }
                    KeyPress::Input { prompt, prefill, action } => {
                        return input_loop(terminal, state, prompt, prefill, action);
                    }
                    KeyPress::Redraw | KeyPress::Unrecognised => {}
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Tab, kind: KeyEventKind::Press, .. }) => {
                match crate::action::press_key(state, '\t') {
                    KeyPress::Execute(f) => {
                        ratatui::restore();
                        return f();
                    }
                    KeyPress::Input { prompt, prefill, action } => {
                        return input_loop(terminal, state, prompt, prefill, action);
                    }
                    KeyPress::Redraw | KeyPress::Unrecognised => {}
                }
            }
            _ => {}
        }
    }
}

fn input_loop(
    terminal: &mut Terminal<impl Backend>,
    state: &LeaderState,
    prompt: &str,
    prefill: String,
    action: fn(String) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let mut value = prefill;
    loop {
        terminal.draw(|frame| render_input(frame, state, prompt, &value))?;
        match event::read()? {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => return Ok(()),
            Event::Key(KeyEvent { code: KeyCode::Enter, kind: KeyEventKind::Press, .. }) => {
                ratatui::restore();
                return action(value);
            }
            Event::Key(KeyEvent { code: KeyCode::Backspace, kind: KeyEventKind::Press, .. }) => {
                value.pop();
            }
            Event::Key(KeyEvent { code: KeyCode::Char(c), kind: KeyEventKind::Press, .. }) => {
                value.push(c);
            }
            _ => {}
        }
    }
}

fn render_input(frame: &mut Frame, state: &LeaderState, prompt: &str, value: &str) {
    let area = frame.area();
    let block = popup_block(format!(" {} {} ", state.icon, state.label));
    let inner_width = area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let mut lines: Vec<Line> = Vec::new();
    for chunk in state.nodes.chunks(COLS) {
        let mut spans: Vec<Span> = Vec::new();
        for (i, node) in chunk.iter().enumerate() {
            let is_last = i + 1 == chunk.len();
            let icon = match &node.kind {
                keymap::KeyNodeKind::Group { icon, .. } if !icon.is_empty() => format!("{} ", icon),
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
    lines.push(Line::from(Span::styled(
        "─".repeat(inner_width.saturating_sub(4)),
        Style::default().fg(comment()),
    )));
    lines.push(Line::from(vec![
        Span::styled(format!(" {}: ", prompt), Style::default().fg(comment())),
        Span::styled(value.to_owned(), Style::default().fg(fg())),
        Span::styled("█", Style::default().fg(mauve())),
    ]));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn render(frame: &mut Frame, state: &LeaderState) {
    let nodes = state.nodes;
    let area = frame.area();

    let block = popup_block(format!(" {} {} ", state.icon, state.label));
    let inner_width = area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let mut lines: Vec<Line> = Vec::new();
    for chunk in nodes.chunks(COLS) {
        let mut spans: Vec<Span> = Vec::new();
        for (i, node) in chunk.iter().enumerate() {
            let is_last = i + 1 == chunk.len();
            let icon = match &node.kind {
                keymap::KeyNodeKind::Group { icon, .. } if !icon.is_empty() => format!("{} ", icon),
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

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

// ---------------------------------------------------------------------------
// pick — interactive list for window/session selection
// ---------------------------------------------------------------------------

pub fn pick(prompt: &str, groups: &[PickGroup]) -> anyhow::Result<Option<(usize, usize)>> {
    let mut key_map: Vec<(usize, usize)> = Vec::new();
    for (gi, group) in groups.iter().enumerate() {
        for ii in 0..group.items.len() {
            key_map.push((gi, ii));
        }
    }

    let initial_cursor = groups
        .first()
        .map(|g| g.initial_cursor.min(g.items.len().saturating_sub(1)))
        .unwrap_or(0);

    let mut terminal = ratatui::init();
    let result = pick_loop(&mut terminal, prompt, groups, &key_map, initial_cursor);
    ratatui::restore();
    result
}

fn pick_loop(
    terminal: &mut Terminal<impl Backend>,
    prompt: &str,
    groups: &[PickGroup],
    key_map: &[(usize, usize)],
    initial_cursor: usize,
) -> anyhow::Result<Option<(usize, usize)>> {
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
            Event::Key(KeyEvent { code: KeyCode::Tab, kind: KeyEventKind::Press, .. }) |
            Event::Key(KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Press, .. }) => {
                if !key_map.is_empty() {
                    cursor = (cursor + 1) % key_map.len();
                }
            }
            Event::Key(KeyEvent { code: KeyCode::BackTab, kind: KeyEventKind::Press, .. }) |
            Event::Key(KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Press, .. }) => {
                if !key_map.is_empty() {
                    cursor = (cursor + key_map.len() - 1) % key_map.len();
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Char(c), kind: KeyEventKind::Press, .. }) => {
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

    let block = popup_block(format!(" {} ", prompt));

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

    let mut lines: Vec<Line> = Vec::new();
    for (gi, group) in groups.iter().enumerate() {
        if !group.label.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                group.label.clone(),
                Style::default().fg(comment()),
            )]));
        }
        for chunk_start in (0..group.items.len()).step_by(COLS) {
            let chunk_end = (chunk_start + COLS).min(group.items.len());
            let mut spans: Vec<Span> = Vec::new();
            for (col, ii) in (chunk_start..chunk_end).enumerate() {
                let key_char = key_chars.get(&(gi, ii)).copied().unwrap_or('?');
                let is_last = col + 1 == (chunk_end - chunk_start);
                let item = &group.items[ii];
                let focused = cursor_pos == Some((gi, ii));
                spans.extend(slot_spans_str(
                    &key_char.to_string(),
                    &item.label,
                    "",
                    lw,
                    is_last,
                    focused,
                    item.current,
                ));
            }
            lines.push(Line::from(spans));
        }
    }

    frame.render_widget(Paragraph::new(lines).block(block), area);
}
