use crate::app::{App, InputMode};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let (title, color) = match app.input_mode {
        InputMode::Normal => (
            format!(
                " Current mode: Normal. User: {}. Database: {}. Host: {}.",
                app.user, app.db_name, app.host
            ),
            Color::Blue,
        ),
        InputMode::Editing => (
            format!(
                " Current mode: Edit. User: {}. Database: {}. Host: {}.",
                app.user, app.db_name, app.host
            ),
            Color::Magenta,
        ),
    };

    let default_style = Style::default().fg(color);

    let block = Block::default().style(default_style);

    let input = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(input, area);
}
