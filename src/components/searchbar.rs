use crate::app::{App, InputMode, FocusElement};
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style},
    Frame,
};

use unicode_width::UnicodeWidthStr;

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {

    let (render_color, title) = match app.focused_element {
        FocusElement::SearchBar => ( Color::Green, " Search (focused) "),
        _ => ( Color::Red, " Search "),
    };

   let default_style = Style::default().fg(render_color);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(default_style);

    let input = Paragraph::new(app.input.as_ref())
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(input, area);
    f.render_widget(block, area);

    if app.input_mode == InputMode::Editing &&
        app.focused_element == FocusElement::SearchBar {
            f.set_cursor(
                area.x + app.input.width() as u16 + 1,
                area.y + 1,
            )
    }

}

