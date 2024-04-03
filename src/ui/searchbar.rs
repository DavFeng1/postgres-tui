use crate::app::{App, FocusElement, InputMode};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use unicode_width::UnicodeWidthStr;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let (render_color, title) = match app.focused_element {
        FocusElement::SearchBar => (Color::Green, " Search (focused) "),
        _ => (Color::Red, " Search "),
    };

    let default_style = Style::default().fg(render_color);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(default_style);

    let input = Paragraph::new(app.input.clone()).block(Block::default().borders(Borders::ALL));

    f.render_widget(input, area);
    f.render_widget(block, area);

    if app.input_mode == InputMode::Editing && app.focused_element == FocusElement::SearchBar {
        f.set_cursor(area.x + app.input.width() as u16 + 1, area.y + 1)
    }
}
