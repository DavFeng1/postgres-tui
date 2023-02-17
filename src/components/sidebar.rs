use crate::app::{App, FocusElement};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    text::{Span, Spans},
    Frame, buffer::Buffer,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    // Title
    let (render_color, title) = match app.focused_element {

        FocusElement::Sidebar => (Color::Green, format!(" Explorer (focused) {}", app.selected_database)),

        _ => (Color::Red, format!(" Explorer {}", app.selected_database)),
    };
    


    // Fetch data
    let database_list = &app.database_list.items;

    let mut list_items: Vec<ListItem> = vec![];
    let mut table_list: Vec<String> = vec![];

    for database in database_list {
        let database_name = database.name.as_str();

        for table in database.tables.clone() {
            table_list.push(table);
        }

        list_items.push(ListItem::new(database_name));
    }

    
    // Basic stateful widget

    // let items = List::new(list_items)
    //     .block(
    //         Block::default()
    //             .title(title)
    //             .borders(Borders::ALL)
    //             .style(Style::default().fg(render_color)),
    //     )
    //     .highlight_style(Style::default().bg(Color::Blue))
    //     .highlight_symbol(">> ");
    //
    // f.render_stateful_widget(items, area, &mut app.database_list.state);


    // Try building the tree

    
    let border_block1 = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(render_color));


    let items = vec![String::from("hello"), String::from("world")];

    f.render_widget(
        CustomWidget::new(items),
        area,
    );
}


#[derive(Debug, Clone)]
struct CustomWidget {
    items: Vec<String>,
}

impl CustomWidget {
    pub fn new(items: Vec<String>) -> CustomWidget {

        Self {
            items,
        }
    }
}

impl<'a> Widget for CustomWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        println!("Not implemented");
    }
 }




