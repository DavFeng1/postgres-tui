use crate::app::{App, FocusElement};
use crate::postgres::query::get_databases;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, ListItem, List},
    style::{ Color, Style },
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    let mut database_list: Vec<ListItem> = vec!();

    for row in get_databases(&mut app.postgres_client) {
        let database_name: String = row.get(0);
        let list_item = ListItem::new(database_name);

        database_list.push(list_item);
    }

    let (render_color, title) = match app.focused_element {
        FocusElement::Sidebar => ( Color::Green, " Explorer (focused) "),
        _ => ( Color::Red, " Explorer ")
    };


    let items = List::new(database_list).block(Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(render_color)));

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(95),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ]
            .as_ref(),
        )
        .split(vertical_layout[0])[0];


    f.render_widget(items, area);
}

