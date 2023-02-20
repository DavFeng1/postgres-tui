use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};

use super::{database::Database, database_cluster::DatabaseCluster};

#[derive(Debug, Clone)]
pub struct DatabaseTree<'a> {
    block: Option<Block<'a>>,
    cluster: DatabaseCluster,
}

impl<'a> DatabaseTree<'a> {
    pub fn new(databases: Vec<Database>) -> DatabaseTree<'a> {
        Self {
            block: None,
            cluster: DatabaseCluster::new(databases),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> DatabaseTree<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for DatabaseTree<'a> {
    type State = DatabaseCluster;

    fn render(mut self, area: Rect, buf: &mut Buffer, _cluster: &mut DatabaseCluster) {
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

        // Draw each database node
        for database in self.cluster.databases.iter() {
            if y_position_to_draw > inner_area.y + inner_area.height {
                break;
            };

            let content: String = if database.is_focused {
                String::from(">>>") + &database.name
            } else {
                database.name.to_string()
            };

            buf.set_stringn(
                starting_x,
                y_position_to_draw,
                content,
                inner_area.width as usize,
                Style::default(),
            );

            // Draw tables for the current database
            if database.is_connected {
                for table in database.tables.iter() {
                    y_position_to_draw += 1;

                    if y_position_to_draw >= inner_area.y + inner_area.height {
                        break;
                    };

                    let content: String = if table.is_focused {
                        String::from(">>>") + &table.name
                    } else {
                        table.name.to_string()
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
