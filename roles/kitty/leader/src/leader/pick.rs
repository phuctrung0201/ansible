use std::collections::HashMap;

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame, Terminal,
};

use super::{
    dividers::divider_with_vertical_margin,
    layout::{label_width, popup_block, slot_spans_str, top_rect},
    theme::{palette, COLS},
};

pub struct PickItem {
    pub label: String,
    pub focused: bool,
    pub current: bool,
}

pub struct PickGroup {
    pub label: String,
    pub items: Vec<PickItem>,
}

pub fn pick(prompt: &str, groups: &[PickGroup]) -> anyhow::Result<Option<(usize, usize)>> {
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
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return Ok(None),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if let Some(&pos) = key_map.get(cursor) {
                    return Ok(Some(pos));
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                kind: KeyEventKind::Press,
                ..
            }) => {
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
    let t = palette();
    let area = frame.area();
    let block = popup_block();
    let div_w = (area.width as usize).saturating_sub(8).max(8);

    let inner_width = area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let key_chars: HashMap<(usize, usize), char> = key_map
        .iter()
        .enumerate()
        .filter_map(|(i, &(gi, ii))| {
            if i < 9 {
                Some(((gi, ii), (b'1' + i as u8) as char))
            } else {
                None
            }
        })
        .collect();

    let cursor_pos = key_map.get(cursor).copied();

    let mut lines = divider_with_vertical_margin(prompt, div_w, t.mauve);

    for (gi, group) in groups.iter().enumerate() {
        if !group.label.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                group.label.clone(),
                Style::default().fg(t.comment).bg(t.dracula_bg),
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
            .style(Style::default().bg(t.dracula_bg))
            .block(block),
        list_area,
    );
}
