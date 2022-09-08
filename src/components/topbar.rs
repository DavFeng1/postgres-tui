use crate::app::{App, FocusElement};
use tui::{
    backend::Backend,
    layout::{Alignment, Direction, Constraint, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Color, Modifier, Style},
    text::Span,
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {

    let size = f.size();

    let text = if app.show_keybinds {

        format!("({:?} Mode) Press k to show keybinds, press q to quit", app.input_mode)

    } else {

        format!("({:?} Mode) Press k to show keybinds, press q to quit", app.input_mode)

    };

    let render_color = if app.focused_element == FocusElement::SearchBar {
            Color::Green
        } else {
            Color::Red
        };

    let title = if app.focused_element == FocusElement::SearchBar {
            " Search (focused) "
        } else {
            " Search "
        };

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]
            .as_ref(),
        )
        .horizontal_margin(1)
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


    let paragraph = Paragraph::new(
        Span::styled(
            text,
            Style::default().add_modifier(Modifier::SLOW_BLINK),
        ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(block, area);
    f.render_widget(paragraph, area)

}

