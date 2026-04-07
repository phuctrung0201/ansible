use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
};

use super::{layout::rule_span, theme::{DRACULA_BG, MAUVE}};

/// Blank row used as vertical margin around section dividers.
pub(crate) fn section_spacer_line() -> Line<'static> {
    Line::from("")
}

/// Horizontal rule with centered label, e.g. `───  windows  ───` (width = display columns).
pub(crate) fn titled_rule_line(title: &str, width: usize) -> Line<'static> {
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
pub(crate) fn divider_with_vertical_margin(title: &str, width: usize) -> Vec<Line<'static>> {
    vec![
        section_spacer_line(),
        titled_rule_line(title, width),
        section_spacer_line(),
    ]
}
