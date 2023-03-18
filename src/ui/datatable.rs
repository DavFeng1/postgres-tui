use crate::app::{App, FocusElement};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let (render_color, title) = match app.focused_element {
        FocusElement::Main => (Color::Green, " Main View (focused) "),
        _ => (Color::Red, " Main View "),
    };

    let default_style = Style::default().fg(render_color);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(default_style);

    match app.cluster.get_current_selected_table() {
        Some(current_table) => {
            let column_names = current_table.columns.clone().join(",");
            let names = Paragraph::new(column_names)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(names, area);
        },
        None => {}
    };

    f.render_widget(block, area);
}
