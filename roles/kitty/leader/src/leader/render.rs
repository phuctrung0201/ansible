use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{action::LeaderState, keynode};

use super::{
    context,
    dividers::divider_with_vertical_margin,
    layout::{label_width, slot_spans, top_rect},
    pills::{banner_pills_line, tab_list_lines, window_pill_lines},
    theme::{
        palette, ACTIONS_TITLE_ICON, COLS, LAUNCHER_SECTION_ICON, TABS_SECTION_ICON,
    },
};

const MAX_TAB_LIST_LINES: usize = 10;

pub(crate) fn render(frame: &mut Frame, state: &LeaderState) {
    let t = palette();
    let nodes = state.nodes;
    let area = frame.area();

    let block = super::layout::popup_block();
    let div_w = (area.width as usize).saturating_sub(8).max(8);
    let header = format!("{} actions", ACTIONS_TITLE_ICON);

    let n_rows = if context::is_launch_group(state) || context::is_tab_list_group(state) {
        0
    } else {
        (nodes.len() as u16).div_ceil(COLS as u16)
    };

    let pill_max_w = (area.width as usize).saturating_sub(4).max(20);
    let current_tab_title = state
        .tab_rows
        .iter()
        .find(|r| r.current)
        .map(|r| r.label.as_str());
    let has_any_banner = state.cwd_pill.is_some()
        || state.kube_pill.is_some()
        || state.git_pill.is_some()
        || current_tab_title.is_some();

    let inner_width_usize = (area.width as usize).saturating_sub(8).max(20);
    let tab_label_w = inner_width_usize.saturating_sub(6).max(12);

    let tab_list_block: Vec<Line<'static>> = if context::is_tab_list_group(state) && !state.tab_rows.is_empty()
    {
        let mut v: Vec<Line<'static>> = Vec::new();
        v.extend(divider_with_vertical_margin(
            &format!("{} tabs", TABS_SECTION_ICON),
            div_w,
            t.mauve,
        ));
        let dim = Style::default()
            .fg(t.comment_bright)
            .bg(t.dracula_bg)
            .add_modifier(Modifier::ITALIC);
        let prefix = Span::styled("  filter · ", Style::default().fg(t.comment_bright).bg(t.dracula_bg));
        if state.tab_filter.is_empty() {
            v.push(Line::from(vec![
                prefix,
                Span::styled("type to fuzzy-match · esc clears query, then root", dim),
            ]));
        } else {
            v.push(Line::from(vec![
                prefix,
                Span::styled(
                    state.tab_filter.clone(),
                    Style::default()
                        .fg(t.fg)
                        .bg(t.dracula_bg)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }
        v.extend(tab_list_lines(
            &state.tab_rows,
            &state.tab_filtered_indices,
            state.tab_cursor,
            MAX_TAB_LIST_LINES,
            tab_label_w,
        ));
        v
    } else {
        Vec::new()
    };

    let mut top_strip: Vec<Line<'static>> = Vec::new();
    if context::is_launch_group(state) && !state.launch_rows.is_empty() {
        top_strip.extend(divider_with_vertical_margin(
            &format!("{} launcher", LAUNCHER_SECTION_ICON),
            div_w,
            t.mauve,
        ));
        top_strip.extend(window_pill_lines(
            &state.launch_rows,
            state.launch_cursor,
            pill_max_w,
            true,
        ));
    }
    let banner_lines = u16::from(has_any_banner);
    let tab_section_lines = tab_list_block.len() as u16;
    let strip_extra = banner_lines + tab_section_lines + top_strip.len() as u16;

    let header_rule_lines: u16 = if context::is_launch_group(state) || context::is_tab_list_group(state)
    {
        0
    } else {
        3
    };
    let popup_height = n_rows + strip_extra + header_rule_lines + 1;
    let popup_area = top_rect(area.width, popup_height, area);

    let inner_width = popup_area.width.saturating_sub(2) as usize;
    let lw = label_width(inner_width);

    let mut lines: Vec<Line> = Vec::new();
    if let Some(line) = banner_pills_line(
        state.cwd_pill.as_deref(),
        state.kube_pill.as_deref(),
        state.git_pill.as_deref(),
        current_tab_title,
        pill_max_w,
    ) {
        lines.push(line);
    }
    lines.extend(tab_list_block);
    lines.extend(top_strip);
    if !context::is_launch_group(state) && !context::is_tab_list_group(state) {
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
                let label = if matches!(&node.kind, keynode::KeyNodeKind::Group { .. }) {
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
            .style(Style::default().bg(t.dracula_bg))
            .block(block),
        popup_area,
    );
}
