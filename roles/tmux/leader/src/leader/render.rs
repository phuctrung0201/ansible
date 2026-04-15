use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_input::Input;

use crate::action::LeaderState;
use crate::keynode;

use super::{
    context,
    dividers::divider_with_vertical_margin,
    layout::{label_width, slot_spans, top_rect},
    pills::{pane_pill_lines, window_pill_lines},
    theme::{
        palette, ACTIONS_TITLE_ICON, COLS, PANES_SECTION_ICON, SESSIONS_SECTION_ICON,
        TABS_SECTION_ICON,
    },
};

fn render_input_paragraph(
    frame: &mut Frame,
    area: Rect,
    input: &Input,
    input_block: Block<'_>,
    text_style: Style,
    show_cursor: bool,
) {
    let inner = input_block.inner(area);
    let width = inner.width.max(1) as usize;
    let scroll = input.visual_scroll(width);
    let scroll_u16 = scroll.min(u16::MAX as usize) as u16;
    frame.render_widget(
        Paragraph::new(input.value())
            .style(text_style)
            .scroll((0, scroll_u16))
            .block(input_block),
        area,
    );
    if show_cursor {
        let col = input.visual_cursor().max(scroll) - scroll;
        let x = inner.x.saturating_add(col.min(u16::MAX as usize) as u16);
        frame.set_cursor_position((x, inner.y));
    }
}

pub(crate) fn render(frame: &mut Frame, state: &LeaderState) {
    let t = palette();
    let nodes = state.nodes;
    let area = frame.area();

    let block = super::layout::popup_block();
    let div_w = (area.width as usize).saturating_sub(8).max(8);
    let header = format!("{} actions", ACTIONS_TITLE_ICON);

    let pending_panel = state.pending_input.is_some();

    let n_rows = if pending_panel || context::is_attach_session_group(state) {
        0u16
    } else {
        COLS as u16
    };

    let pill_max_w = (area.width as usize).saturating_sub(4).max(20);

    let mut session_list_block: Vec<Line<'static>> = Vec::new();
    if context::session_pill_strip_visible(state) {
        session_list_block.extend(divider_with_vertical_margin(
            &format!("{} sessions", SESSIONS_SECTION_ICON),
            div_w,
            t.mauve,
        ));
        if state.session_rows.is_empty() {
            session_list_block.push(Line::from(vec![Span::styled(
                "  (no sessions)",
                Style::default()
                    .fg(t.comment_bright)
                    .bg(t.dracula_bg)
                    .add_modifier(Modifier::ITALIC),
            )]));
        } else {
            session_list_block.extend(window_pill_lines(
                &state.session_rows,
                state.session_cursor,
                pill_max_w,
                true,
            ));
        }
    }

    let mut attach_session_list_block: Vec<Line<'static>> = Vec::new();
    if context::attach_session_section_visible(state) && !state.session_rows.is_empty() {
        attach_session_list_block.extend(divider_with_vertical_margin(
            &format!("{} attach to session", SESSIONS_SECTION_ICON),
            div_w,
            t.mauve,
        ));
        attach_session_list_block.extend(window_pill_lines(
            &state.session_rows,
            state.session_cursor,
            pill_max_w,
            true,
        ));
    }

    // Pending rename prompt: section rule + bordered input field + hint row (bottom).
    let input_lines: u16 = if pending_panel { 3 + 3 + 1 } else { 0 };

    let notice_lines = u16::from(state.notice.is_some());
    let session_section_lines = session_list_block.len() as u16;
    let attach_session_section_lines = attach_session_list_block.len() as u16;
    let strip_extra = notice_lines + session_section_lines + attach_session_section_lines;

    let header_rule_lines: u16 = if pending_panel || context::is_attach_session_group(state) {
        0
    } else {
        3
    };
    let root_tab_pills: Option<Vec<Line<'static>>> =
        if context::window_tab_strip_visible(state) && !state.tab_rows.is_empty() {
            Some(window_pill_lines(
                &state.tab_rows,
                state.root_window_cursor,
                pill_max_w,
                true,
            ))
        } else {
            None
        };
    let window_body_rows: u16 = if context::window_tab_strip_visible(state) {
        if state.tab_rows.is_empty() {
            1
        } else {
            root_tab_pills.as_ref().map_or(0, |v| v.len() as u16)
        }
    } else {
        0
    };
    let window_section_lines: u16 = if context::window_tab_strip_visible(state) {
        3 + window_body_rows
    } else {
        0
    };
    let root_pane_pills: Option<Vec<Line<'static>>> =
        if context::pane_section_visible(state) && !state.pane_rows.is_empty() {
            Some(pane_pill_lines(
                &state.pane_rows,
                state.root_pane_cursor,
                pill_max_w,
                true,
            ))
        } else {
            None
        };
    let pane_body_rows: u16 = if context::pane_section_visible(state) {
        if state.pane_rows.is_empty() {
            1
        } else {
            root_pane_pills.as_ref().map_or(0, |v| v.len() as u16)
        }
    } else {
        0
    };
    let pane_section_lines: u16 = if context::pane_section_visible(state) {
        3 + pane_body_rows
    } else {
        0
    };
    let popup_height = n_rows
        + strip_extra
        + window_section_lines
        + pane_section_lines
        + header_rule_lines
        + input_lines
        + 1;
    let popup_area = top_rect(area.width, popup_height, area);

    let inner_width = popup_area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let mut lines: Vec<Line> = Vec::new();
    if let Some(ref msg) = state.notice {
        let dim = Style::default()
            .fg(t.comment_bright)
            .bg(t.dracula_bg)
            .add_modifier(Modifier::ITALIC);
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(msg.clone(), dim),
        ]));
    }
    lines.extend(session_list_block);
    lines.extend(attach_session_list_block);
    if context::window_tab_strip_visible(state) {
        lines.extend(divider_with_vertical_margin(
            &format!("{} windows", TABS_SECTION_ICON),
            div_w,
            t.mauve,
        ));
        if state.tab_rows.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "  (no windows)",
                Style::default()
                    .fg(t.comment_bright)
                    .bg(t.dracula_bg)
                    .add_modifier(Modifier::ITALIC),
            )]));
        } else if let Some(pills) = root_tab_pills {
            lines.extend(pills);
        }
    }
    if context::pane_section_visible(state) {
        lines.extend(divider_with_vertical_margin(
            &format!("{} panes", PANES_SECTION_ICON),
            div_w,
            t.mauve,
        ));
        if state.pane_rows.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "  (no panes)",
                Style::default()
                    .fg(t.comment_bright)
                    .bg(t.dracula_bg)
                    .add_modifier(Modifier::ITALIC),
            )]));
        } else if let Some(pills) = root_pane_pills {
            lines.extend(pills);
        }
    }
    if !pending_panel && !context::is_attach_session_group(state) {
        lines.extend(divider_with_vertical_margin(&header, div_w, t.mauve));
        for row in 0..COLS {
            let mut spans: Vec<Span> = Vec::new();
            for col in 0..COLS {
                let idx = col * COLS + row;
                if let Some(node) = nodes.get(idx) {
                    let is_last = col + 1 == COLS
                        || (col + 1..COLS).all(|c| nodes.get(c * COLS + row).is_none());
                    let icon = match &node.kind {
                        keynode::KeyNodeKind::Group { icon, .. } if !icon.is_empty() => {
                            format!("{} ", icon)
                        }
                        _ => String::new(),
                    };
                    let label = match &node.kind {
                        keynode::KeyNodeKind::Group { .. } => format!("{}+", node.label),
                        _ => node.label.to_string(),
                    };
                    spans.extend(slot_spans(node.key, &label, &icon, lw, is_last, false));
                }
            }
            lines.push(Line::from(spans));
        }
    }

    let inner = block.inner(popup_area);
    let bg = Style::default().bg(t.dracula_bg);

    if let Some(ref pending) = state.pending_input {
        let mut head: Vec<Line<'static>> = Vec::new();
        let section_title = format!(
            "{} {}",
            ACTIONS_TITLE_ICON,
            pending.prompt.trim_end_matches(':')
        );
        head.extend(divider_with_vertical_margin(
            section_title.as_str(),
            div_w,
            t.mauve,
        ));
        let hint_dim = Style::default()
            .fg(t.comment_bright)
            .bg(t.dracula_bg)
            .add_modifier(Modifier::ITALIC);
        let head_h = head.len() as u16;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(head_h),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(inner);
        frame.render_widget(
            Paragraph::new(Line::from(""))
                .style(bg)
                .block(block.clone()),
            popup_area,
        );
        frame.render_widget(Paragraph::new(head).style(bg), chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(t.teal))
            .style(Style::default().bg(t.dracula_bg));
        let field_style = Style::default()
            .fg(t.fg)
            .bg(t.dracula_bg)
            .add_modifier(Modifier::BOLD);
        render_input_paragraph(
            frame,
            chunks[1],
            &pending.input,
            input_block,
            field_style,
            true,
        );
        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(
                "type, then enter · esc · cancel",
                hint_dim,
            )]))
            .style(bg)
            .alignment(Alignment::Right),
            chunks[2],
        );
        return;
    }

    frame.render_widget(Paragraph::new(lines).style(bg).block(block), popup_area);
}
