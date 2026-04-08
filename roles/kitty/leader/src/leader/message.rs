use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    style::Style,
    text::Line,
    widgets::Paragraph,
    Frame, Terminal,
};

use super::{
    dividers::divider_with_vertical_margin,
    layout::popup_block,
    theme::palette,
};

pub fn show_message(title: &str, body: &str) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let result = message_loop(&mut terminal, title, body);
    ratatui::restore();
    result
}

fn message_loop(
    terminal: &mut Terminal<impl Backend>,
    title: &str,
    body: &str,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render_message(frame, title, body))?;
        if let Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            ..
        }) = event::read()?
        {
            return Ok(());
        }
    }
}

fn render_message(frame: &mut Frame, title: &str, body: &str) {
    let t = palette();
    let area = frame.area();
    let div_w = (area.width as usize).saturating_sub(8).max(8);
    let block = popup_block();
    let mut lines = divider_with_vertical_margin(title, div_w, t.mauve);
    lines.push(Line::from(body.to_owned()));
    frame.render_widget(
        Paragraph::new(lines)
            .style(Style::default().fg(t.fg).bg(t.dracula_bg))
            .centered()
            .block(block),
        area,
    );
}
