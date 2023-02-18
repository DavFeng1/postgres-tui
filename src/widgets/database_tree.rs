use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};

use super::database_state::DatabaseState;

#[derive(Debug, Clone)]
pub struct DatabaseTree<'a> {
    block: Option<Block<'a>>,
    state: DatabaseState,
}

impl<'a> DatabaseTree<'a> {
    pub fn new(databases: Vec<String>) -> DatabaseTree<'a> {
        Self {
            block: None,
            state: DatabaseState::with_database_list(databases),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> DatabaseTree<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for DatabaseTree<'a> {
    type State = DatabaseState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut DatabaseState) {
        let inner_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);

                b.render(area, buf);

                inner_area
            }
            None => area,
        };

        // Padding for border
        let mut y_position_to_draw = inner_area.y;
        let starting_x = inner_area.x;

        let focused_element = match state.focused_element {
            Some(db) => db,
            None => 0,
        };

        let selected_database = match state.selected_database {
            Some(db) => db,
            None => 0,
        };

        // Draw each database node
        for (i, database_name) in self.state.database_list.iter().enumerate() {
            if y_position_to_draw > inner_area.y + inner_area.height {
                break;
            };

            let is_element_focused = i == focused_element;
            let is_database_selected = i == selected_database;

            let content: String = if is_element_focused {
                String::from(">>>") + &database_name
            } else {
                database_name.to_string()
            };

            buf.set_stringn(
                starting_x,
                y_position_to_draw,
                content,
                inner_area.width as usize,
                Style::default(),
            );

            // Draw tables for the current database
            if is_database_selected {
                let tables_for_database = match state.tables_map.get(database_name) {
                    Some(result) => result.clone(),
                    None => Vec::new(),
                };

                for table_name in tables_for_database {
                    y_position_to_draw += 1;

                    if y_position_to_draw >= inner_area.y + inner_area.height {
                        break;
                    };

                    let content: String = if is_element_focused {
                        String::from(">>>") + &table_name
                    } else {
                        table_name.to_string()
                    };


                    // Padding for nesting
                    buf.set_stringn(
                        starting_x + 3,
                        y_position_to_draw,
                        content,
                        inner_area.width as usize,
                        Style::default(),
                    );
                }
            }
            y_position_to_draw += 1;
        }
    }
}
