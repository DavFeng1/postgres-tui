use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    style::{Color, Style},
    Frame,
};

pub fn app_layout<B: Backend>(f: &mut Frame<B>) {

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ]
            .as_ref(),
        )
        .horizontal_margin(1)
        .vertical_margin(1)
        .split(f.size());

    let default_style = Style::default().fg(Color::Red);

    let block = Block::default()
        .title(" Explorer ")
        .borders(Borders::ALL)
        .style(default_style);

    f.render_widget(block, chunks[0]);

    let block = Block::default()
        .title(" Main View ")
        .borders(Borders::ALL)
        .style(default_style);

    f.render_widget(block, chunks[1]);
}



