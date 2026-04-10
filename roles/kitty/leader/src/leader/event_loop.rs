use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    Terminal,
};

use crate::action::{KeyPress, LeaderState};

use super::{context, render::render};

pub(crate) fn run(
    terminal: &mut Terminal<impl Backend>,
    state: &mut LeaderState,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_tab_list_group(state) && !state.tab_filter.is_empty() {
                    state.tab_filter.clear();
                    state.recompute_tab_filter_keep(None);
                    continue;
                }
                if context::is_tab_list_group(state) {
                    state.return_to_root();
                    continue;
                }
                return Ok(());
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_tab_list_group(state) && !state.tab_filtered_indices.is_empty() {
                    let row_i = state.tab_filtered_indices[state.tab_cursor];
                    let id = state.tab_rows[row_i].id;
                    ratatui::restore();
                    return crate::action::focus_tab_from_leader(id);
                }
                if context::is_launch_group(state) && !state.launch_rows.is_empty() {
                    let idx = state.launch_rows[state.launch_cursor].id as usize;
                    ratatui::restore();
                    return crate::action::execute_launch_at(idx);
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_tab_list_group(state) && !state.tab_filtered_indices.is_empty() {
                    let len = state.tab_filtered_indices.len();
                    state.tab_cursor = (state.tab_cursor + 1) % len;
                    continue;
                }
                if context::is_launch_group(state) && !state.launch_rows.is_empty() {
                    state.launch_cursor = (state.launch_cursor + 1) % state.launch_rows.len();
                    continue;
                }
                if context::is_launch_group(state) {
                    continue;
                }
                if context::is_root(state) {
                    match crate::action::press_key(state, '\t') {
                        KeyPress::Redraw => continue,
                        KeyPress::Execute(f) => {
                            ratatui::restore();
                            return f();
                        }
                        KeyPress::Unrecognised => continue,
                    }
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_tab_list_group(state) && !state.tab_filtered_indices.is_empty() {
                    let len = state.tab_filtered_indices.len();
                    state.tab_cursor = (state.tab_cursor + len - 1) % len;
                    continue;
                }
                if context::is_launch_group(state) && !state.launch_rows.is_empty() {
                    let len = state.launch_rows.len();
                    state.launch_cursor = (state.launch_cursor + len - 1) % len;
                    continue;
                }
                if context::is_launch_group(state) {
                    continue;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_tab_list_group(state) && !state.tab_rows.is_empty() && !state.tab_filter.is_empty()
                {
                    let keep = state.selected_tab_id();
                    state.tab_filter.pop();
                    state.recompute_tab_filter_keep(keep);
                    continue;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_tab_list_group(state) && !state.tab_rows.is_empty() {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let j = idx - 1;
                            if j < state.tab_filtered_indices.len() {
                                let row_i = state.tab_filtered_indices[j];
                                let id = state.tab_rows[row_i].id;
                                ratatui::restore();
                                return crate::action::focus_tab_from_leader(id);
                            }
                        }
                    }
                    if !c.is_control() {
                        let keep = state.selected_tab_id();
                        state.tab_filter.push(c);
                        state.recompute_tab_filter_keep(keep);
                    }
                    continue;
                }
                if context::is_launch_group(state) {
                    if !state.launch_rows.is_empty() {
                        if let Some(d) = c.to_digit(10) {
                            let idx = d as usize;
                            if (1..=9).contains(&idx) {
                                let j = idx - 1;
                                if j < state.launch_rows.len() {
                                    let li = state.launch_rows[j].id as usize;
                                    ratatui::restore();
                                    return crate::action::execute_launch_at(li);
                                }
                            }
                        }
                    }
                    continue;
                }
                match crate::action::press_key(state, c) {
                    KeyPress::Execute(f) => {
                        ratatui::restore();
                        return f();
                    }
                    KeyPress::Redraw | KeyPress::Unrecognised => {}
                }
            }
            _ => {}
        }
    }
}
