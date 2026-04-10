use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::action::LeaderWindowRow;

use super::{
    layout::popup_gap,
    theme::{
        palette, CWD_PILL_ICON, GIT_PILL_ICON, KUBE_PILL_ICON, ROUND_CAP_L, ROUND_CAP_R,
        TAB_TITLE_PILL_ICON,
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
    let t = palette();
    if selected {
        Style::default()
            .fg(t.dracula_bg)
            .bg(t.mauve)
            .add_modifier(Modifier::BOLD)
    } else if kitty_focused {
        Style::default()
            .fg(t.teal)
            .bg(t.pill_bg)
            .add_modifier(Modifier::BOLD)
    } else if recent {
        Style::default()
            .fg(t.dracula_bg)
            .bg(t.yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(t.fg).bg(t.pill_bg)
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

fn cwd_pill_spans(cwd: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let t = palette();
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = CWD_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(cwd, max_text);
    let inner = format!(" {} {} ", CWD_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(t.dracula_bg)
        .bg(t.green)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(t.green).bg(t.dracula_bg);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

fn kube_pill_spans(ctx: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let t = palette();
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = KUBE_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(ctx, max_text);
    let inner = format!(" {} {} ", KUBE_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(t.dracula_bg)
        .bg(t.kube_pill_bg)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(t.kube_pill_bg).bg(t.dracula_bg);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

fn tab_title_pill_spans(title: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let t = palette();
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = TAB_TITLE_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(title, max_text);
    let inner = format!(" {} {} ", TAB_TITLE_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(t.dracula_bg)
        .bg(t.mauve)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(t.mauve).bg(t.dracula_bg);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

fn git_pill_spans(branch: &str, max_line_width: usize) -> Vec<Span<'static>> {
    let t = palette();
    let max_inner = max_line_width.saturating_sub(4).clamp(8, 120);
    let icon_reserve = GIT_PILL_ICON.chars().count().saturating_add(1);
    let max_text = max_inner.saturating_sub(icon_reserve).max(4);
    let inner_text = truncate_pill_label(branch, max_text);
    let inner = format!(" {} {} ", GIT_PILL_ICON, inner_text);
    let mid = Style::default()
        .fg(t.dracula_bg)
        .bg(t.orange)
        .add_modifier(Modifier::BOLD);
    let cap = Style::default().fg(t.orange).bg(t.dracula_bg);
    vec![
        Span::styled(ROUND_CAP_L, cap),
        Span::styled(inner, mid),
        Span::styled(ROUND_CAP_R, cap),
    ]
}

fn spans_total_width(spans: &[Span]) -> usize {
    spans.iter().map(|s| s.width()).sum()
}

/// Top banner: current tab and cwd on the left; git and kube on the right (flex gap between).
pub(crate) fn banner_pills_prefix_spans(
    cwd: Option<&str>,
    kube: Option<&str>,
    git: Option<&str>,
    current_tab_title: Option<&str>,
    max_line_width: usize,
) -> Option<Vec<Span<'static>>> {
    let n_left =
        usize::from(current_tab_title.is_some()) + usize::from(cwd.is_some());
    let n_right = usize::from(git.is_some()) + usize::from(kube.is_some());
    if n_left + n_right == 0 {
        return None;
    }

    let t = palette();
    let fill = Style::default().bg(t.dracula_bg);

    let (w_left, w_right) = match (n_left, n_right) {
        (0, nr) => {
            let w = (max_line_width.saturating_sub(nr.saturating_sub(1)) / nr).max(18);
            (0usize, w)
        }
        (nl, 0) => {
            let w = (max_line_width.saturating_sub(nl.saturating_sub(1)) / nl).max(18);
            (w, 0usize)
        }
        (nl, nr) => {
            let half = max_line_width / 2;
            let wl = (half.saturating_sub(nl.saturating_sub(1)) / nl).max(18);
            let right_budget = max_line_width.saturating_sub(half).saturating_sub(1);
            let wr = (right_budget.saturating_sub(nr.saturating_sub(1)) / nr).max(18);
            (wl, wr)
        }
    };

    let mut left: Vec<Span<'static>> = Vec::new();
    if let Some(tab) = current_tab_title {
        if !left.is_empty() {
            left.push(popup_gap());
        }
        left.extend(tab_title_pill_spans(tab, w_left));
    }
    if let Some(c) = cwd {
        if !left.is_empty() {
            left.push(popup_gap());
        }
        left.extend(cwd_pill_spans(c, w_left));
    }

    let mut right: Vec<Span<'static>> = Vec::new();
    if let Some(g) = git {
        if !right.is_empty() {
            right.push(popup_gap());
        }
        right.extend(git_pill_spans(g, w_right));
    }
    if let Some(k) = kube {
        if !right.is_empty() {
            right.push(popup_gap());
        }
        right.extend(kube_pill_spans(k, w_right));
    }

    if n_left > 0 && n_right == 0 {
        return Some(left);
    }
    if n_left == 0 && n_right > 0 {
        let rw = spans_total_width(&right);
        let pad = max_line_width.saturating_sub(rw);
        let mut out = Vec::new();
        if pad > 0 {
            out.push(Span::styled(" ".repeat(pad), fill));
        }
        out.extend(right);
        return Some(out);
    }

    let lw = spans_total_width(&left);
    let rw = spans_total_width(&right);
    let gap = max_line_width.saturating_sub(lw + rw).max(1);
    let mut out = left;
    out.push(Span::styled(" ".repeat(gap), fill));
    out.extend(right);
    Some(out)
}

/// Top row banner pills (read‑only): tab · cwd — git · kube.
pub(crate) fn banner_pills_line(
    cwd: Option<&str>,
    kube: Option<&str>,
    git: Option<&str>,
    current_tab_title: Option<&str>,
    max_line_width: usize,
) -> Option<Line<'static>> {
    let spans = banner_pills_prefix_spans(cwd, kube, git, current_tab_title, max_line_width)?;
    Some(Line::from(spans))
}

/// Scrollable plain-text tab list (no pills); `cursor` indexes into `filtered`.
pub(crate) fn tab_list_lines(
    tab_rows: &[LeaderWindowRow],
    filtered: &[usize],
    cursor: usize,
    max_visible: usize,
    max_label_width: usize,
) -> Vec<Line<'static>> {
    let t = palette();
    let n = filtered.len();
    if n == 0 {
        return vec![Line::from(vec![Span::styled(
            "  (no matching tabs)",
            Style::default().fg(t.comment_bright).bg(t.dracula_bg),
        )])];
    }
    let max_vis = max_visible.max(1);
    let skip = if n <= max_vis {
        0
    } else {
        cursor
            .saturating_sub(max_vis / 2)
            .min(n.saturating_sub(max_vis))
    };
    let mut lines: Vec<Line<'static>> = Vec::new();
    if skip > 0 {
        lines.push(Line::from(vec![Span::styled(
            format!("  ··· {} above", skip),
            Style::default().fg(t.comment_bright).bg(t.dracula_bg),
        )]));
    }
    let take = (n - skip).min(max_vis);
    for j in skip..skip + take {
        let row_i = filtered[j];
        let row = &tab_rows[row_i];
        let is_sel = j == cursor;
        let bullet = if is_sel { "› " } else { "  " };
        let label = truncate_pill_label(&row.label, max_label_width);
        let mut style = Style::default().bg(t.dracula_bg);
        if is_sel {
            style = style.fg(t.pink).add_modifier(Modifier::BOLD);
        } else if row.current {
            style = style.fg(t.teal).add_modifier(Modifier::BOLD);
        } else {
            style = style.fg(t.fg);
        }
        lines.push(Line::from(vec![
            Span::styled(bullet, style),
            Span::styled(label, style),
        ]));
    }
    if skip + take < n {
        let rem = n - (skip + take);
        lines.push(Line::from(vec![Span::styled(
            format!("  ··· {} below", rem),
            Style::default().fg(t.comment_bright).bg(t.dracula_bg),
        )]));
    }
    lines
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
