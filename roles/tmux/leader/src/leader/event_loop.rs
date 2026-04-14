use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    Terminal,
};
use tui_input::backend::crossterm::EventHandler;

use crate::action::{KeyPress, LeaderState};

use super::{context, render::render};

pub(crate) fn run(
    terminal: &mut Terminal<impl Backend>,
    state: &mut LeaderState,
) -> anyhow::Result<()> {
    loop {
        terminal
            .draw(|frame| render(frame, state))
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let event = event::read()?;
        if !context::is_input_mode(state) {
            state.notice = None;
        }
        if context::is_input_mode(state) {
            match &event {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    state.pending_input = None;
                    continue;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if let Some(pending) = state.pending_input.take() {
                        let value = pending.input.value().to_string();
                        let confirm = !value.is_empty() || pending.allow_empty_confirm;
                        if confirm {
                            super::term::restore_global();
                            return (pending.confirm)(value);
                        }
                    }
                    continue;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Tab,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    continue;
                }
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press | KeyEventKind::Repeat,
                    ..
                }) => {
                    if let Some(ref mut p) = state.pending_input {
                        p.input.handle_event(&event);
                    }
                    continue;
                }
                _ => continue,
            }
        }
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_move_session_group(state) {
                    state.return_to_windows();
                    continue;
                }
                if !context::is_root(state) {
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
                if context::is_move_session_group(state) && !state.session_rows.is_empty() {
                    if let Some(name) = state.selected_session_name() {
                        super::term::restore_global();
                        return crate::action::move_window_to_session(name.trim());
                    }
                }
                if context::pane_section_visible(state) && !state.pane_rows.is_empty() {
                    let n = state.pane_rows.len().min(24);
                    let pid = state.pane_rows[state.root_pane_cursor.min(n - 1)]
                        .pane_id
                        .clone();
                    super::term::restore_global();
                    return crate::action::focus_pane_from_leader(&pid);
                }
                if context::window_tab_strip_visible(state) && !state.tab_rows.is_empty() {
                    let n = state.tab_rows.len().min(24);
                    if n > 0 {
                        let id = state.tab_rows[state.root_window_cursor].id;
                        super::term::restore_global();
                        return crate::action::focus_tab_from_leader(id);
                    }
                }
                if context::is_root(state)
                    && context::root_session_section_visible(state)
                    && !state.session_rows.is_empty()
                {
                    if let Some(name) = state.selected_session_name() {
                        super::term::restore_global();
                        return crate::action::focus_session_from_leader(name);
                    }
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_input_mode(state) {
                    continue;
                }
                if context::is_move_session_group(state) && !state.session_rows.is_empty() {
                    let n = state.session_rows.len().min(24);
                    if n > 0 {
                        state.session_cursor = (state.session_cursor + 1) % n;
                    }
                    continue;
                }
                if context::pane_section_visible(state) && !state.pane_rows.is_empty() {
                    let n = state.pane_rows.len().min(24);
                    state.root_pane_cursor = (state.root_pane_cursor + 1) % n;
                    continue;
                }
                if context::window_tab_strip_visible(state) {
                    let n = state.tab_rows.len().min(24);
                    if n > 0 {
                        state.root_window_cursor = (state.root_window_cursor + 1) % n;
                        continue;
                    }
                }
                if context::is_root(state)
                    && context::root_session_section_visible(state)
                    && !state.session_rows.is_empty()
                {
                    let n = state.session_rows.len().min(24);
                    if n > 0 {
                        state.session_cursor = (state.session_cursor + 1) % n;
                    }
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_input_mode(state) {
                    continue;
                }
                if context::is_move_session_group(state) && !state.session_rows.is_empty() {
                    let n = state.session_rows.len().min(24);
                    if n > 0 {
                        state.session_cursor = (state.session_cursor + n - 1) % n;
                    }
                    continue;
                }
                if context::pane_section_visible(state) && !state.pane_rows.is_empty() {
                    let n = state.pane_rows.len().min(24);
                    state.root_pane_cursor = (state.root_pane_cursor + n - 1) % n;
                    continue;
                }
                if context::window_tab_strip_visible(state) {
                    let n = state.tab_rows.len().min(24);
                    if n > 0 {
                        state.root_window_cursor = (state.root_window_cursor + n - 1) % n;
                        continue;
                    }
                }
                if context::is_root(state)
                    && context::root_session_section_visible(state)
                    && !state.session_rows.is_empty()
                {
                    let n = state.session_rows.len().min(24);
                    if n > 0 {
                        state.session_cursor = (state.session_cursor + n - 1) % n;
                    }
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            }) => {
                if context::is_move_session_group(state) && !state.session_rows.is_empty() {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let i = idx - 1;
                            let n = state.session_rows.len().min(24);
                            if i < n {
                                let name = state.session_rows[i].label.clone();
                                super::term::restore_global();
                                return crate::action::move_window_to_session(name.trim());
                            }
                        }
                    }
                    continue;
                }
                if context::pane_section_visible(state) && !state.pane_rows.is_empty() {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let i = idx - 1;
                            if i < state.pane_rows.len() {
                                let pid = state.pane_rows[i].pane_id.clone();
                                super::term::restore_global();
                                return crate::action::focus_pane_from_leader(&pid);
                            }
                        }
                    }
                }
                if context::window_tab_strip_visible(state) {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let i = idx - 1;
                            if i < state.tab_rows.len() {
                                let id = state.tab_rows[i].id;
                                super::term::restore_global();
                                return crate::action::focus_tab_from_leader(id);
                            }
                        }
                    }
                }
                if context::is_root(state)
                    && context::root_session_section_visible(state)
                    && !state.session_rows.is_empty()
                {
                    if let Some(d) = c.to_digit(10) {
                        let idx = d as usize;
                        if (1..=9).contains(&idx) {
                            let i = idx - 1;
                            let n = state.session_rows.len().min(24);
                            if i < n {
                                let name = state.session_rows[i].label.clone();
                                super::term::restore_global();
                                return crate::action::focus_session_from_leader(name);
                            }
                        }
                    }
                }
                match crate::action::press_key(state, c) {
                    KeyPress::Execute(f) => {
                        super::term::restore_global();
                        return f();
                    }
                    KeyPress::Notice(msg) => {
                        state.notice = Some(msg);
                    }
                    KeyPress::OpenInput {
                        prompt,
                        initial,
                        confirm,
                        allow_empty_confirm,
                    } => {
                        state.enter_input_mode(prompt, initial, confirm, allow_empty_confirm);
                    }
                    KeyPress::Redraw | KeyPress::Unrecognised => {}
                }
            }
            _ => {}
        }
    }
}
