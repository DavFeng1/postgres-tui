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

        FocusElement::Sidebar => (Color::Green, format!(" Explorer (focused) {}", app.selected_database)),

        _ => (Color::Red, format!(" Explorer {}", app.selected_database)),
    };

    let database_list = &app.database_list.items;

    let mut list_items: Vec<ListItem> = vec![];
    
    let mut table_list: Vec<ListItem> = vec![];

    for database in database_list {
        let database_name = database.name.as_str();

        list_items.push(ListItem::new(database_name));

        for table in database.tables.clone() {
            table_list.push(ListItem::new(table));
        }
    }

    
    let tables = List::new(table_list);

    let items = List::new(list_items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .style(Style::default().fg(render_color)),
        )
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol(">> ");

    f.render_stateful_widget(tables, area, &mut app.database_list.state);
    f.render_stateful_widget(items, area, &mut app.database_list.state);
}
