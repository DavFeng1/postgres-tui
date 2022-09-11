
use crate::app::{App, FocusElement};
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    style::{ Color, Style},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {

    let (render_color, title) = match app.focused_element {
        FocusElement::Main => (Color:: Green, " Main View (focused) "),
        _ => (Color::Red, " Main View ")
    };

    let default_style = Style::default().fg(render_color);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(default_style);

    f.render_widget(block, area);
}

