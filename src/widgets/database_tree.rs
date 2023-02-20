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

struct BufferLine {
    x: u16,
    y: u16,
    content: String,
    width: usize,
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

        let mut y_position_to_draw = inner_area.y;
        let starting_x = inner_area.x;

        let mut lines_to_draw: Vec<BufferLine> = Vec::new();

        for database in self.cluster.databases.iter() {
            if y_position_to_draw > inner_area.y + inner_area.height {
                break;
            };

            let content: String = if database.is_focused {
                String::from(">>>") + &database.name
            } else {
                database.name.to_string()
            };

            lines_to_draw.push(BufferLine {
                x: starting_x,
                y: y_position_to_draw,
                content,
                width: inner_area.width as usize,
            });

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

                    lines_to_draw.push(BufferLine {
                        x: starting_x + 3,
                        y: y_position_to_draw,
                        content,
                        width: inner_area.width as usize,
                    });
                }
            }
            y_position_to_draw += 1;
        }

        for line in lines_to_draw {
            buf.set_stringn(line.x, line.y, line.content, line.width, Style::default());
        }
    }
}
