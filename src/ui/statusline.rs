use crate::app::{App, InputMode};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let (title, color) = match app.input_mode {
        InputMode::Normal => (" Current mode: Normal", Color::Blue),
        InputMode::Editing => (" Current mode: Edit ", Color::Magenta),
    };

    let default_style = Style::default().fg(color);

    let block = Block::default().style(default_style);

    let input = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(input, area);
}
