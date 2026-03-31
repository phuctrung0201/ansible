use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    Frame, Terminal,
};

use crate::{action::{LeaderState, KeyPress}, keymap};

// ---------------------------------------------------------------------------
// Dracula palette
// ---------------------------------------------------------------------------
const PURPLE: Color = Color::Rgb(189, 147, 249);
const CYAN_BG: Color = Color::Rgb(139, 233, 253);
const FG: Color = Color::Rgb(248, 248, 242);
const COMMENT: Color = Color::Rgb(98, 114, 164);

const COLS: usize = 4;

// ---------------------------------------------------------------------------
// PickGroup — secondary selection
// ---------------------------------------------------------------------------

pub struct PickItem {
    pub label: String,
    pub focused: bool,
}

pub struct PickGroup {
    pub label: String,
    pub items: Vec<PickItem>,
}

// ---------------------------------------------------------------------------
// Shared slot rendering helpers
// ---------------------------------------------------------------------------

/// Returns the label column width given a popup's inner width.
fn label_width(inner_width: usize) -> usize {
    let slot_width = inner_width / COLS;
    slot_width.saturating_sub(3) // badge(1) + leading-space(1) + inter-slot gap(1)
}

/// Two spans for a single key-badge + label slot.
fn slot_spans(key: char, label: &str, icon: &str, lw: usize, is_last: bool, focused: bool) -> [Span<'static>; 2] {
    let trailing = if is_last { 0 } else { 2 };
    let icon_chars = icon.chars().count();
    let text = format!(
        " → {}{:<width$}{:>trail$}",
        icon,
        label,
        "",
        width = lw.saturating_sub(icon_chars),
        trail = trailing,
    );
    let label_style = if focused {
        Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(FG)
    };
    [
        Span::styled(key.to_string(), Style::default().fg(CYAN_BG).add_modifier(Modifier::BOLD)),
        Span::styled(text, label_style),
    ]
}

fn popup_block(title: String) -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(PURPLE))
        .title_top(
            Line::from(Span::styled(title, Style::default().fg(COMMENT))).centered(),
        )
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
    let [_, popup_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .areas(area);

    let block = popup_block(format!(" {} ", title));
    frame.render_widget(
        Paragraph::new(body.to_owned())
            .centered()
            .block(block),
        popup_area,
    );
}

pub fn run() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = LeaderState::new();
    let result = event_loop(&mut terminal, &mut state);
    ratatui::restore();
    result
}

// ---------------------------------------------------------------------------
// Event loop
// ---------------------------------------------------------------------------

fn event_loop(
    terminal: &mut Terminal<impl Backend>,
    state: &mut LeaderState,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        match event::read()? {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => return Ok(()),
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            }) => match crate::action::press_key(state, c) {
                KeyPress::Execute(f) => {
                    ratatui::restore();
                    return f();
                }
                KeyPress::Redraw | KeyPress::Unrecognised => {}
            },
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

    let popup_height = (nodes.len() as u16).div_ceil(COLS as u16) + 2;
    let [_, popup_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(popup_height),
    ])
    .areas(area);

    let block = popup_block(format!(" {} {} ", state.icon, state.label));

    let inner_width = popup_area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let mut lines: Vec<Line> = Vec::new();
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

    frame.render_widget(Paragraph::new(lines).block(block), popup_area);
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

    let content_rows: u16 = groups
        .iter()
        .map(|g| {
            let header = if g.label.is_empty() { 0 } else { 1 };
            header + (g.items.len() as u16).div_ceil(COLS as u16)
        })
        .sum();
    let popup_height = content_rows + 2;

    let mut terminal = ratatui::init();
    let result = pick_loop(&mut terminal, prompt, groups, &key_map, popup_height);
    ratatui::restore();
    result
}

fn pick_loop(
    terminal: &mut Terminal<impl Backend>,
    prompt: &str,
    groups: &[PickGroup],
    key_map: &[(usize, usize)],
    popup_height: u16,
) -> anyhow::Result<Option<(usize, usize)>> {
    loop {
        terminal.draw(|frame| render_pick(frame, prompt, groups, key_map, popup_height))?;
        match event::read()? {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => return Ok(None),
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            }) => {
                if c.is_ascii_lowercase() {
                    let idx = (c as u8 - b'a') as usize;
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
    popup_height: u16,
) {
    let area = frame.area();

    let [_, popup_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(popup_height),
    ])
    .areas(area);

    let block = popup_block(format!(" {} ", prompt));

    let inner_width = popup_area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let key_chars: std::collections::HashMap<(usize, usize), char> = key_map
        .iter()
        .enumerate()
        .filter_map(|(i, &(gi, ii))| {
            if i < 26 { Some(((gi, ii), (b'a' + i as u8) as char)) } else { None }
        })
        .collect();

    let mut lines: Vec<Line> = Vec::new();

    for (gi, group) in groups.iter().enumerate() {
        if !group.label.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                group.label.clone(),
                Style::default().fg(COMMENT),
            )]));
        }

        for chunk_start in (0..group.items.len()).step_by(COLS) {
            let chunk_end = (chunk_start + COLS).min(group.items.len());
            let chunk_len = chunk_end - chunk_start;
            let mut spans: Vec<Span> = Vec::new();
            for (col, ii) in (chunk_start..chunk_end).enumerate() {
                let key_char = key_chars.get(&(gi, ii)).copied().unwrap_or('?');
                let is_last = col + 1 == chunk_len;
                let item = &group.items[ii];
                spans.extend(slot_spans(key_char, &item.label, "", lw, is_last, item.focused));
            }
            lines.push(Line::from(spans));
        }
    }

    frame.render_widget(Paragraph::new(lines).block(block), popup_area);
}
