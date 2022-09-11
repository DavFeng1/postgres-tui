use crate::app::{App, InputMode};
use tui::{
    backend::Backend,
    layout::{Alignment, Direction, Constraint, Layout},
    widgets::{Block, Paragraph},
    style::{Color, Style},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {

    let size = f.size();

    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(95),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(size)[1];

    let title = match app.input_mode {
        InputMode::Normal => " Current mode: Normal ",
        InputMode::Editing => " Current mode: Edit ",
    };

    let default_style = Style::default().fg(Color::Green);

    let block = Block::default()
        .title_alignment(Alignment::Center)
        .style(default_style);

    let input = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(input, area);
}

