use crate::app::{App, InputMode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Clear, Paragraph},
    style::{ Color, Style},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App, percent_x: u16, percent_y: u16) {

    let size = f.size();

    let popup_block = Block::default().borders(Borders::ALL).title(
        match app.input_mode {
            InputMode::Normal => "Normal mode",
            InputMode::Editing => "Editing mode"
        }
    );

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Green),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(popup_block);


    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(size);

    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(vertical_layout[1])[1];

    f.render_widget(Clear, area);
    f.render_widget(input, area);

    match app.input_mode {

        InputMode::Normal => {},

        InputMode::Editing => {
            f.set_cursor(
                area.x + app.input.chars().count() as u16 + 1,
                area.y + 1,
            )
        }
    }
}

