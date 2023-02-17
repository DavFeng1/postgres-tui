use crate::app::{App, FocusElement};
use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Corner, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    // Title
    let (render_color, title) = match app.focused_element {
        FocusElement::Sidebar => (
            Color::Green,
            format!(" Explorer (focused) {}", app.selected_database),
        ),

        _ => (Color::Red, format!(" Explorer {}", app.selected_database)),
    };

    // Fetch data
    let database_list = &app.database_list.items;

    let mut list_items: Vec<ListItem> = vec![];
    let mut table_list: Vec<String> = vec![];
    let mut database_string_list: Vec<String> = vec![];

    for database in database_list {
        let database_name = database.name.as_str();

        for table in database.tables.clone() {
            table_list.push(table);
        }

        database_string_list.push(String::from(database_name));
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

    let border_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(render_color));

    let items = vec![String::from("hello"), String::from("world")];

    f.render_widget(
        CustomWidget::new(database_string_list).block(border_block),
        area
    );
}

#[derive(Debug, Clone)]
struct CustomWidget<'a> {
    block: Option<Block<'a>>,
    items: Vec<String>,
    start_corner: Corner,
}

impl<'a> CustomWidget<'a> {
    pub fn new(items: Vec<String>) -> CustomWidget<'a> {
        Self {
            block: None,
            items,
            start_corner: Corner::TopLeft,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> CustomWidget<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for CustomWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {

        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        for item in self.items.iter_mut()
        {
            let area = Rect {
                x: 0,
                y: 0,
                width: list_area.width,
                height: 40 as u16,
            };


            buf.set_stringn(
                area.left() as u16,
                area.top(),
                item,
                40 as usize,
                Style::default(),
            );
        }
    }
}



