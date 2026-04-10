use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::Block,
};

use super::theme::{palette, COLS, KEY_WIDTH};

pub(crate) fn key_display(key: char) -> String {
    let s = match key {
        ' ' => "space".to_string(),
        '\t' => "tab".to_string(),
        _ => key.to_string(),
    };
    format!("{:>KEY_WIDTH$}", s)
}

/// Returns the label column width given a popup's inner width.
pub(crate) fn label_width(inner_width: usize) -> usize {
    let slot_width = inner_width / COLS;
    slot_width.saturating_sub(KEY_WIDTH + 3 + 6) // badge(KEY_WIDTH) + " → "(3) + trailing(6)
}

pub(crate) fn slot_spans_str(
    key: &str,
    label: &str,
    icon: &str,
    lw: usize,
    is_last: bool,
    focused: bool,
    current: bool,
) -> [Span<'static>; 2] {
    let trailing = if is_last { 0 } else { 6 };
    let icon_chars = icon.chars().count();
    let max_label = lw.saturating_sub(icon_chars);
    let label: std::borrow::Cow<str> = if label.chars().count() > max_label {
        label
            .chars()
            .take(max_label.saturating_sub(1))
            .chain(std::iter::once('…'))
            .collect::<String>()
            .into()
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
    let t = palette();
    let label_style = if focused {
        Style::default().fg(t.pink).add_modifier(Modifier::BOLD)
    } else if current {
        Style::default().fg(t.yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(t.fg)
    };
    let key_str = format!("{:>KEY_WIDTH$}", key);
    let key_style = if current {
        Style::default().fg(t.yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(t.teal).add_modifier(Modifier::BOLD)
    };
    [
        Span::styled(key_str, key_style),
        Span::styled(text, label_style),
    ]
}

/// Two spans for a single key-badge + label slot.
pub(crate) fn slot_spans(
    key: char,
    label: &str,
    icon: &str,
    lw: usize,
    is_last: bool,
    focused: bool,
) -> [Span<'static>; 2] {
    slot_spans_str(&key_display(key), label, icon, lw, is_last, focused, false)
}

pub(crate) fn top_rect(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.x,
        y: area.y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

pub(crate) fn popup_block() -> Block<'static> {
    let t = palette();
    Block::new()
        .style(Style::default().bg(t.dracula_bg))
        .padding(ratatui::widgets::Padding::new(2, 2, 1, 0))
}

pub(crate) fn popup_gap() -> Span<'static> {
    Span::styled(" ", Style::default().bg(palette().dracula_bg))
}

pub(crate) fn rule_span(s: String) -> Span<'static> {
    let t = palette();
    Span::styled(s, Style::default().fg(t.comment_bright).bg(t.dracula_bg))
}
