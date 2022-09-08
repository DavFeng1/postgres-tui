
use crate::app::{App, FocusElement};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    style::{ Color, Style},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ]
            .as_ref(),
        )
        .horizontal_margin(1)
        .split(f.size());

    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]
        )
        .split(horizontal_layout[1]);


    let render_color = if app.focused_element == FocusElement::Main {
            Color::Green
        } else {
            Color::Red
        };

    let title = if app.focused_element == FocusElement::Main {
            " Main View (focused) "
        } else {
            " Main View "
        };

    let default_style = Style::default().fg(render_color);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(default_style);

    f.render_widget(block, area[1]);
}

