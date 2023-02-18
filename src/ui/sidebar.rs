use crate::{
    app::{App, FocusElement},
    widgets::database_tree::DatabaseTree,
};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
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
    let database_list = &app.database_state.databases;

    // Try building the tree
    let border_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(render_color));

    f.render_stateful_widget(
        DatabaseTree::new(database_list.to_vec()).block(border_block),
        area,
        &mut app.database_state,
    );
}
