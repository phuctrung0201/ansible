use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::action::{LeaderPaneRow, LeaderWindowRow};

use super::{
    layout::popup_gap,
    theme::{palette, ROUND_CAP_L, ROUND_CAP_R},
};

pub(crate) fn truncate_pill_label(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        s.chars()
            .take(max_chars.saturating_sub(1))
            .chain(std::iter::once('…'))
            .collect()
    }
}

pub(crate) fn pill_style(selected: bool, term_focused: bool, recent: bool) -> Style {
    let t = palette();
    if selected {
        Style::default()
            .fg(t.pill_fg)
            .bg(t.mauve)
            .add_modifier(Modifier::BOLD)
    } else if term_focused {
        Style::default()
            .fg(t.teal)
            .bg(t.pill_bg)
            .add_modifier(Modifier::BOLD)
    } else if recent {
        Style::default()
            .fg(t.pill_fg)
            .bg(t.yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(t.fg)
            .bg(t.pill_bg)
            .add_modifier(Modifier::BOLD)
    }
}

/// Pill fill color for caps (must match middle `bg` for non-selected states).
pub(crate) fn pill_cap_fill_color(selected: bool, current: bool) -> ratatui::style::Color {
    let t = palette();
    if selected {
        t.mauve
    } else if current {
        t.yellow
    } else {
        t.pill_bg
    }
}

fn window_pill_line(line: Line<'static>, center: bool) -> Line<'static> {
    if center {
        line.alignment(Alignment::Center)
    } else {
        line
    }
}

/// Wrapped horizontal lines of window “pills” for the root header.
pub(crate) fn window_pill_lines(
    rows: &[LeaderWindowRow],
    cursor: usize,
    max_line_width: usize,
    center: bool,
) -> Vec<Line<'static>> {
    const MAX_WINDOWS: usize = 24;
    const MIN_CHARS: usize = 6;
    let t = palette();
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
            out.push(window_pill_line(Line::from(line_spans), center));
            line_spans = Vec::new();
            used = 0;
        }
        if !line_spans.is_empty() {
            line_spans.push(popup_gap());
            used += 1;
        }
        let mid = pill_style(sel, row.focused, row.current);
        let cap_style = Style::default().fg(bg).bg(t.dracula_bg);
        line_spans.push(Span::styled(ROUND_CAP_L, cap_style));
        line_spans.push(Span::styled(inner, mid));
        line_spans.push(Span::styled(ROUND_CAP_R, cap_style));
        used += w;
    }

    if !line_spans.is_empty() {
        out.push(window_pill_line(Line::from(line_spans), center));
    }
    if rows.len() > MAX_WINDOWS {
        out.push(window_pill_line(
            Line::from(vec![Span::styled(
                format!("… +{} more", rows.len() - MAX_WINDOWS),
                Style::default().fg(t.comment_bright).bg(t.dracula_bg),
            )]),
            center,
        ));
    }
    out
}

/// Pane “pills” for the **`p`** group (same layout as [`window_pill_lines`]).
pub(crate) fn pane_pill_lines(
    rows: &[LeaderPaneRow],
    cursor: usize,
    max_line_width: usize,
    center: bool,
) -> Vec<Line<'static>> {
    const MAX_PANES: usize = 24;
    const MIN_CHARS: usize = 6;
    let t = palette();
    let max_chars = (max_line_width / 5).clamp(MIN_CHARS, 22);

    let mut out: Vec<Line<'static>> = Vec::new();
    let mut line_spans: Vec<Span<'static>> = Vec::new();
    let mut used = 0usize;

    for (i, row) in rows.iter().enumerate().take(MAX_PANES) {
        let label = truncate_pill_label(&row.label, max_chars);
        let inner: String = if i < 9 {
            format!(" {} {} ", i + 1, label)
        } else {
            format!(" {} ", label)
        };
        let sel = i == cursor;
        let bg = pill_cap_fill_color(sel, row.current);
        let w = inner.chars().count() + 2;
        let gap = if line_spans.is_empty() { 0 } else { 1 };

        if used + gap + w > max_line_width && !line_spans.is_empty() {
            out.push(window_pill_line(Line::from(line_spans), center));
            line_spans = Vec::new();
            used = 0;
        }
        if !line_spans.is_empty() {
            line_spans.push(popup_gap());
            used += 1;
        }
        let mid = pill_style(sel, false, row.current);
        let cap_style = Style::default().fg(bg).bg(t.dracula_bg);
        line_spans.push(Span::styled(ROUND_CAP_L, cap_style));
        line_spans.push(Span::styled(inner, mid));
        line_spans.push(Span::styled(ROUND_CAP_R, cap_style));
        used += w;
    }

    if !line_spans.is_empty() {
        out.push(window_pill_line(Line::from(line_spans), center));
    }
    if rows.len() > MAX_PANES {
        out.push(window_pill_line(
            Line::from(vec![Span::styled(
                format!("… +{} more", rows.len() - MAX_PANES),
                Style::default().fg(t.comment_bright).bg(t.dracula_bg),
            )]),
            center,
        ));
    }
    out
}
