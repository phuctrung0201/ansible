use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::action::LeaderWindowRow;

use super::{
    layout::popup_gap,
    theme::{
        CWD_PILL_BG, CWD_PILL_ICON, DRACULA_BG, GIT_PILL_ICON, GREEN, KUBE_PILL_ICON, MAUVE,
        ORANGE, PILL_BG, ROUND_CAP_L, ROUND_CAP_R, YELLOW, FG,
    },
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

pub(crate) fn pill_style(selected: bool, kitty_focused: bool, recent: bool) -> Style {
    if selected {
        Style::default()
            .fg(DRACULA_BG)
            .bg(MAUVE)
            .add_modifier(Modifier::BOLD)
    } else if kitty_focused {
        Style::default()
            .fg(MAUVE)
            .bg(PILL_BG)
            .add_modifier(Modifier::BOLD)
    } else if recent {
        Style::default()
            .fg(DRACULA_BG)
            .bg(YELLOW)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(FG).bg(PILL_BG)
    }
}

/// Pill fill color for caps (must match middle `bg` for non-selected states).
pub(crate) fn pill_cap_fill_color(selected: bool, current: bool) -> ratatui::style::Color {
    if selected {
        MAUVE
    } else if current {
        YELLOW
    } else {
        PILL_BG
    }
}

fn cwd_pill_spans(cwd: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = CWD_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(cwd, max_text);
    let inner = format!(" {} {} ", CWD_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(DRACULA_BG)
        .bg(GREEN)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(GREEN).bg(DRACULA_BG);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

fn kube_pill_spans(ctx: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = KUBE_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(ctx, max_text);
    let inner = format!(" {} {} ", KUBE_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(DRACULA_BG)
        .bg(CWD_PILL_BG)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(CWD_PILL_BG).bg(DRACULA_BG);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

fn git_pill_spans(branch: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = GIT_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(branch, max_text);
    let inner = format!(" {} {} ", GIT_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(DRACULA_BG)
        .bg(ORANGE)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(ORANGE).bg(DRACULA_BG);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

/// Cwd / git / kube spans for the top row (no alignment). Used standalone (centered) or prefixed to window/tab/launcher pills.
pub(crate) fn banner_pills_prefix_spans(
    cwd: Option<&str>,
    kube: Option<&str>,
    git: Option<&str>,
    max_line_width: usize,
) -> Option<Vec<Span<'static>>> {
    let n = usize::from(cwd.is_some()) + usize::from(kube.is_some()) + usize::from(git.is_some());
    if n == 0 {
        return None;
    }
    if n == 1 {
        let spans = match (cwd, kube, git) {
            (Some(c), None, None) => cwd_pill_spans(c, max_line_width),
            (None, Some(k), None) => kube_pill_spans(k, max_line_width),
            (None, None, Some(g)) => git_pill_spans(g, max_line_width),
            _ => return None,
        };
        return Some(spans);
    }
    let gaps = n - 1;
    let w = (max_line_width.saturating_sub(gaps) / n).max(18);
    let mut spans: Vec<Span<'static>> = Vec::new();
    if let Some(c) = cwd {
        if !spans.is_empty() {
            spans.push(popup_gap());
        }
        spans.extend(cwd_pill_spans(c, w));
    }
    if let Some(g) = git {
        if !spans.is_empty() {
            spans.push(popup_gap());
        }
        spans.extend(git_pill_spans(g, w));
    }
    if let Some(k) = kube {
        if !spans.is_empty() {
            spans.push(popup_gap());
        }
        spans.extend(kube_pill_spans(k, w));
    }
    Some(spans)
}

/// Centered top row: cwd / git / kube status pills (read‑only), above windows/tabs/launcher lists.
pub(crate) fn banner_pills_line(
    cwd: Option<&str>,
    kube: Option<&str>,
    git: Option<&str>,
    max_line_width: usize,
) -> Option<Line<'static>> {
    let spans = banner_pills_prefix_spans(cwd, kube, git, max_line_width)?;
    Some(Line::from(spans).alignment(Alignment::Center))
}

/// Wrapped horizontal lines of window “pills” for the root header.
pub(crate) fn window_pill_lines(
    rows: &[LeaderWindowRow],
    cursor: usize,
    max_line_width: usize,
) -> Vec<Line<'static>> {
    const MAX_WINDOWS: usize = 24;
    const MIN_CHARS: usize = 6;
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
            out.push(Line::from(line_spans));
            line_spans = Vec::new();
            used = 0;
        }
        if !line_spans.is_empty() {
            line_spans.push(popup_gap());
            used += 1;
        }
        let mid = pill_style(sel, row.focused, row.current);
        let cap_style = Style::default().fg(bg).bg(DRACULA_BG);
        line_spans.push(Span::styled(ROUND_CAP_L, cap_style));
        line_spans.push(Span::styled(inner, mid));
        line_spans.push(Span::styled(ROUND_CAP_R, cap_style));
        used += w;
    }

    if !line_spans.is_empty() {
        out.push(Line::from(line_spans));
    }
    if rows.len() > MAX_WINDOWS {
        out.push(Line::from(vec![Span::styled(
            format!("… +{} more", rows.len() - MAX_WINDOWS),
            Style::default().fg(super::theme::COMMENT).bg(DRACULA_BG),
        )]));
    }
    out
}
