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

#[derive(Debug, Clone, Default)]
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

        let x = inner_area.x;
        let mut y = inner_area.y;

        let mut lines_to_draw: Vec<BufferLine> = Vec::new();
        let mut current_focused_index_position: usize = 0;

        for (i, database) in self.cluster.databases.iter().enumerate() {
            if y > inner_area.y + inner_area.height {
                break;
            };

            let content: String = if database.is_focused {
                current_focused_index_position = i;
                String::from(">>>") + &database.name
            } else {
                database.name.to_string()
            };

            lines_to_draw.push(BufferLine {
                x,
                y,
                content,
                width: inner_area.width as usize,
            });

            // Draw tables for the current database
            if database.is_connected {
                for (j, table) in database.tables.iter().enumerate() {
                    y += 1;

                    if y >= inner_area.y + inner_area.height {
                        break;
                    };

                    let content: String = if table.is_focused {
                        current_focused_index_position = i + j + 1;
                        String::from(">>>") + &table.name
                    } else {
                        table.name.to_string()
                    };

                    lines_to_draw.push(BufferLine {
                        x: x + 3,
                        y,
                        content,
                        width: inner_area.width as usize,
                    });
                }
            }
            y += 1;
        }

        let height_of_tree = (inner_area.height - inner_area.y) as usize;
        let y_center_of_tree = height_of_tree as usize / 2;
        let total_lines = lines_to_draw.len();
        let y_focused_element = current_focused_index_position + inner_area.y as usize;

        if y_focused_element > y_center_of_tree && total_lines > height_of_tree as usize {
            println!("focused element: {}", y_focused_element);
            println!("y_center_of_tree: {}, ", y_center_of_tree);
            println!("total_lines: {}", total_lines);
            println!("height_of_tree: {}", height_of_tree);
            // The cursor position is greater than halfway down the tree
            // Shift the entries to draw such that the current focused element is in the middle.
            //
            // Caclculate how far past the middlepoint the cursor is
            // and shift the lines to draw by that amount so that the cursor at the current element
            // will be at the center of the tree

            // How many elements past the middle are we
            let how_far_past_middle = y_focused_element - y_center_of_tree;

            // Begin the shifted array at how_far_past_middle until how_far_past_middle +
            // height_of_tree
            let shifted_lines = lines_to_draw[how_far_past_middle..height_of_tree + 1].to_vec();

            // let mut y = inner_area.y;
            for line in lines_to_draw {
                buf.set_stringn(line.x, line.y, line.content, line.width, Style::default());
                // y += 1;
            }
        } else {
            for line in lines_to_draw {
                buf.set_stringn(line.x, line.y, line.content, line.width, Style::default());
            }
        }
    }
}
