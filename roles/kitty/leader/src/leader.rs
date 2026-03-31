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

// ---------------------------------------------------------------------------
// PickGroup — secondary selection
// ---------------------------------------------------------------------------

pub struct PickGroup {
    pub label: String,
    pub items: Vec<String>,
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

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

    // Full width, anchored to bottom
    let popup_height = (nodes.len() as u16).div_ceil(4) + 2;
    let [_, popup_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(popup_height),
    ])
    .areas(area);

    // Build title
    let title = format!(" {} {} ", state.icon, state.label);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(PURPLE))
        .title_top(
            Line::from(Span::styled(title, Style::default().fg(COMMENT)))
                .centered(),
        );

    // Each slot gets an equal share of the inner width; items per row scales
    // with terminal width so there's always breathing room between entries.
    let inner_width = popup_area.width.saturating_sub(2) as usize;
    let cols: usize = 4;
    let slot_width = inner_width / cols;
    // Gap between key badge and label, plus between slots
    let badge_width = 1; // single char key
    let label_width = slot_width.saturating_sub(badge_width + 2); // 2 = leading space + trailing gap

    let mut lines: Vec<Line> = Vec::new();
    for chunk in nodes.chunks(cols) {
        let mut spans: Vec<Span> = Vec::new();
        for (i, node) in chunk.iter().enumerate() {
            let key_badge = Span::styled(
                node.key.to_string(),
                Style::default()
                    .fg(CYAN_BG)
                    .add_modifier(Modifier::BOLD),
            );
            let is_group = matches!(&node.kind, keymap::KeyNodeKind::Group { .. });
            let icon = match &node.kind {
                keymap::KeyNodeKind::Group { icon, .. } if !icon.is_empty() => {
                    format!("{} ", icon)
                }
                _ => String::new(),
            };
            let label = if is_group {
                format!("{}+", node.label)
            } else {
                node.label.to_string()
            };
            let trailing = if i + 1 < chunk.len() { 2 } else { 0 };
            let label_text = format!(
                " {}{:<width$}{:>trail$}",
                icon,
                label,
                "",
                width = label_width.saturating_sub(icon.chars().count()),
                trail = trailing,
            );
            let label_span = Span::styled(label_text, Style::default().fg(FG));
            spans.push(key_badge);
            spans.push(label_span);
        }
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}

// ---------------------------------------------------------------------------
// pick — secondary selection for tab_switch / move_tab_to_window
// ---------------------------------------------------------------------------

pub fn pick(
    prompt: &str,
    groups: &[PickGroup],
) -> anyhow::Result<Option<(usize, usize)>> {
    // Build flat key assignment: a-z across all groups' items
    let mut key_map: Vec<(usize, usize)> = Vec::new(); // (group_idx, item_idx)
    for (gi, group) in groups.iter().enumerate() {
        for ii in 0..group.items.len() {
            key_map.push((gi, ii));
        }
    }

    // Compute popup height: per group = 1 (header) + ceil(items/4) rows + 2 border rows total
    let content_rows: u16 = groups
        .iter()
        .map(|g| 1 + (g.items.len() as u16).div_ceil(4))
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

    let title = format!(" {} ", prompt);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(PURPLE))
        .title_top(
            Line::from(Span::styled(title, Style::default().fg(COMMENT)))
                .centered(),
        );

    let inner_width = popup_area.width.saturating_sub(2) as usize;
    let cols: usize = 4;
    let slot_width = inner_width / cols;
    let badge_width = 1;
    let label_width = slot_width.saturating_sub(badge_width + 2);

    // Build key_char lookup: (group_idx, item_idx) -> char
    let key_chars: std::collections::HashMap<(usize, usize), char> = key_map
        .iter()
        .enumerate()
        .filter_map(|(i, &(gi, ii))| {
            let c = (b'a' + i as u8) as char;
            if i < 26 { Some(((gi, ii), c)) } else { None }
        })
        .collect();

    let mut lines: Vec<Line> = Vec::new();

    for (gi, group) in groups.iter().enumerate() {
        lines.push(Line::from(vec![Span::styled(
            group.label.clone(),
            Style::default().fg(COMMENT),
        )]));

        for chunk_start in (0..group.items.len()).step_by(cols) {
            let chunk_end = (chunk_start + cols).min(group.items.len());
            let mut spans: Vec<Span> = Vec::new();
            for (col, ii) in (chunk_start..chunk_end).enumerate() {
                let key_char = key_chars.get(&(gi, ii)).copied().unwrap_or('?');
                let key_badge = Span::styled(
                    key_char.to_string(),
                    Style::default()
                        .fg(CYAN_BG)
                        .add_modifier(Modifier::BOLD),
                );
                let trailing = if col + 1 < chunk_end - chunk_start { 2 } else { 0 };
                let label_text = format!(
                    " {:<width$}{:>trail$}",
                    group.items[ii],
                    "",
                    width = label_width,
                    trail = trailing,
                );
                let label_span = Span::styled(label_text, Style::default().fg(FG));
                spans.push(key_badge);
                spans.push(label_span);
            }
            lines.push(Line::from(spans));
        }
    }

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}
