use crate::app::{App, FocusElement};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let (render_color, title) = match app.focused_element {
        FocusElement::Sidebar => (Color::Green, " Explorer (focused) "),
        _ => (Color::Red, " Explorer "),
    };

    let database_list = &app.database_state.items.items;

    let mut list_items: Vec<ListItem> = vec![];
    for database in database_list {
        let database_name = database.as_str();
        list_items.push(ListItem::new(database_name))
    }

    let items = List::new(list_items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .style(Style::default().fg(render_color)),
        )
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, area, &mut app.database_state.items.state);
}
