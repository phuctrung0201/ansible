use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_input::Input;

use crate::action::{LeaderState, LeaderView, LeaderWindowRow};
use crate::keynode;

use super::{
    context,
    dividers::divider_with_vertical_margin,
    layout::{label_width, slot_spans, top_rect},
    palette::Palette,
    pills::{banner_pills_line, pane_pill_lines, tab_list_lines, window_pill_lines},
    theme::{
        palette, ACTIONS_TITLE_ICON, COLS, LAUNCHER_SECTION_ICON, PANES_SECTION_ICON,
        SESSIONS_SECTION_ICON, TABS_SECTION_ICON,
    },
};

const PICKER_LIST_MAX_LINES: usize = 10;

/// Session / launcher picker: divider, fuzzy filter line, scrollable list (no numbers).
fn filtered_picker_lines(
    t: &Palette,
    div_w: usize,
    list_label_width: usize,
    section_title: &str,
    filter_empty_hint: &'static str,
    filter: &str,
    no_match: &'static str,
    rows: &[LeaderWindowRow],
    filtered_indices: &[usize],
    cursor: usize,
) -> Vec<Line<'static>> {
    let mut v: Vec<Line<'static>> = Vec::new();
    v.extend(divider_with_vertical_margin(section_title, div_w, t.mauve));
    let dim = Style::default()
        .fg(t.comment_bright)
        .bg(t.dracula_bg)
        .add_modifier(Modifier::ITALIC);
    let prefix = Span::styled("  filter · ", Style::default().fg(t.comment_bright).bg(t.dracula_bg));
    if filter.is_empty() {
        v.push(Line::from(vec![prefix, Span::styled(filter_empty_hint, dim)]));
    } else {
        v.push(Line::from(vec![
            prefix,
            Span::styled(
                filter.to_string(),
                Style::default()
                    .fg(t.fg)
                    .bg(t.dracula_bg)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }
    v.extend(tab_list_lines(
        rows,
        filtered_indices,
        cursor,
        PICKER_LIST_MAX_LINES,
        list_label_width,
        no_match,
    ));
    v
}

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

    let n_rows = if pending_panel
        || context::is_launch_group(state)
        || context::is_session_list_group(state)
    {
        0
    } else {
        (nodes.len() as u16).div_ceil(COLS as u16)
    };

    let pill_max_w = (area.width as usize).saturating_sub(4).max(20);
    let inner_width_usize = (area.width as usize).saturating_sub(8).max(20);
    let picker_list_label_w = inner_width_usize.saturating_sub(6).max(12);
    let has_any_banner = state.kube_pill.is_some() || state.git_pill.is_some();

    const SESSION_FILTER_HINT: &str =
        "type to fuzzy-match · Tab · Enter · esc clears query, then root";
    let session_list_block: Vec<Line<'static>> =
        if context::session_list_panel_visible(state) && !state.session_rows.is_empty() {
            filtered_picker_lines(
                &t,
                div_w,
                picker_list_label_w,
                &format!("{} list sessions", SESSIONS_SECTION_ICON),
                SESSION_FILTER_HINT,
                &state.session_filter,
                "no matching sessions",
                &state.session_rows,
                &state.session_filtered_indices,
                state.session_cursor,
            )
        } else {
            Vec::new()
        };

    let launcher_list_block: Vec<Line<'static>> =
        if context::is_launch_group(state) && !state.launch_rows.is_empty() {
            filtered_picker_lines(
                &t,
                div_w,
                picker_list_label_w,
                &format!("{} launcher", LAUNCHER_SECTION_ICON),
                "type to fuzzy-match · Tab · Enter · esc clears filter, then root",
                &state.launch_filter,
                "no matching apps",
                &state.launch_rows,
                &state.launch_filtered_indices,
                state.launch_cursor,
            )
        } else {
            Vec::new()
        };
    // Pending rename prompt: section rule + bordered input field + hint row (bottom).
    let input_lines: u16 = if pending_panel {
        3 + 3 + 1
    } else {
        0
    };

    let banner_lines = u16::from(has_any_banner);
    let notice_lines = u16::from(state.notice.is_some());
    let session_section_lines = session_list_block.len() as u16;
    let launcher_section_lines = launcher_list_block.len() as u16;
    let strip_extra = banner_lines
        + notice_lines
        + session_section_lines
        + launcher_section_lines;

    let header_rule_lines: u16 = if pending_panel
        || context::is_launch_group(state)
        || context::is_session_list_group(state)
    {
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
    if let Some(line) = banner_pills_line(
        state.git_pill.as_deref(),
        state.kube_pill.as_deref(),
        pill_max_w,
    ) {
        lines.push(line);
    }
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
    lines.extend(launcher_list_block);
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
    if !pending_panel
        && !context::is_launch_group(state)
        && !context::is_session_list_group(state)
    {
        lines.extend(divider_with_vertical_margin(&header, div_w, t.mauve));
        for chunk in nodes.chunks(COLS) {
            let mut spans: Vec<Span> = Vec::new();
            for (i, node) in chunk.iter().enumerate() {
                let is_last = i + 1 == chunk.len();
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
            lines.push(Line::from(spans));
        }
    }

    let inner = block.inner(popup_area);
    let bg = Style::default().bg(t.dracula_bg);

    if state.view == LeaderView::Normal {
        if let Some(ref pending) = state.pending_input {
            let mut head: Vec<Line<'static>> = Vec::new();
            if let Some(line) = banner_pills_line(
                state.git_pill.as_deref(),
                state.kube_pill.as_deref(),
                pill_max_w,
            ) {
                head.push(line);
            }
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
                Paragraph::new(Line::from("")).style(bg).block(block.clone()),
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
    }

    frame.render_widget(Paragraph::new(lines).style(bg).block(block), popup_area);
}
