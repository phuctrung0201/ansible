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
                code: KeyCode::Esc, ..
            }) => return Ok(()),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_root(state) && !state.window_rows.is_empty() {
                    let id = state.window_rows[state.window_cursor].id;
                    ratatui::restore();
                    return crate::action::focus_window_from_leader(id);
                }
                if context::is_tab_group(state) && !state.tab_rows.is_empty() {
                    let id = state.tab_rows[state.tab_cursor].id;
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
                if context::is_root(state) && !state.window_rows.is_empty() {
                    state.window_cursor = (state.window_cursor + 1) % state.window_rows.len();
                    continue;
                }
                if context::is_tab_group(state) && !state.tab_rows.is_empty() {
                    state.tab_cursor = (state.tab_cursor + 1) % state.tab_rows.len();
                    continue;
                }
                if context::is_launch_group(state) && !state.launch_rows.is_empty() {
                    state.launch_cursor = (state.launch_cursor + 1) % state.launch_rows.len();
                    continue;
                }
                match crate::action::press_key(state, '\t') {
                    KeyPress::Execute(f) => {
                        ratatui::restore();
                        return f();
                    }
                    KeyPress::Redraw | KeyPress::Unrecognised => {}
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_root(state) && !state.window_rows.is_empty() {
                    let len = state.window_rows.len();
                    state.window_cursor = (state.window_cursor + len - 1) % len;
                    continue;
                }
                if context::is_tab_group(state) && !state.tab_rows.is_empty() {
                    let len = state.tab_rows.len();
                    state.tab_cursor = (state.tab_cursor + len - 1) % len;
                    continue;
                }
                if context::is_launch_group(state) && !state.launch_rows.is_empty() {
                    let len = state.launch_rows.len();
                    state.launch_cursor = (state.launch_cursor + len - 1) % len;
                    continue;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_root(state) && !state.window_rows.is_empty() {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let j = idx - 1;
                            if j < state.window_rows.len() {
                                let id = state.window_rows[j].id;
                                ratatui::restore();
                                return crate::action::focus_window_from_leader(id);
                            }
                        }
                    }
                }
                if context::is_tab_group(state) && !state.tab_rows.is_empty() {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let j = idx - 1;
                            if j < state.tab_rows.len() {
                                let id = state.tab_rows[j].id;
                                ratatui::restore();
                                return crate::action::focus_tab_from_leader(id);
                            }
                        }
                    }
                }
                if context::is_launch_group(state) && !state.launch_rows.is_empty() {
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
