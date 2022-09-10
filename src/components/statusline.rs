use crate::app::{App, FocusElement};
use tui::{
    backend::Backend,
    layout::{Direction, Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
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
        .horizontal_margin(1)
        .split(size);


    let default_style = Style::default().fg(Color::Green);

    let block = Block::default()
        .title(" Mode ")
        .borders(Borders::ALL)
        .style(default_style);

    f.render_widget(block, area[1]);
}

