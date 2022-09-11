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

    let (render_color, title) = match app.focused_element {
        FocusElement::SearchBar => ( Color::Green, " Search (focused) "),
        _ => ( Color::Red, " Search "),
    };



    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(85),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(size);

    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ]
            .as_ref(),
        )
        .split(vertical_layout[0])[1];

    let default_style = Style::default().fg(render_color);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(default_style);

    let input = Paragraph::new(app.input.as_ref())
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(input, area);
    f.render_widget(block, area);

}

