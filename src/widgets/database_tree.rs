use super::{database::Database, database_cluster::DatabaseCluster};
use std::cmp::{max, min};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, StatefulWidget, Widget},
};

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
    style: Style,
}

impl<'a> StatefulWidget for DatabaseTree<'a> {
    type State = DatabaseCluster;

    // Implement the rendering logic of the database tree.
    //
    // Iterate over all databases and expand tables for the currently
    // connected database (database.is_connected).
    //
    // Calculate the offset for scrolling when # of lines > height_of_tree
    //
    //    [0, 1, 2]              current display (height_of_tree = 3)
    //        ^                  y_center_of_tree
    // [1, 2, 3, 4, 5, 6, 7]     total items (lines_to_draw)
    //        ^                  y_current
    //     ^                     start_of_slice = y_current - height_of_tree / 2
    //           ^               end_of_slice = y_current + height_of_tree / 2
    //    [2, 3, 4]              lines_to_draw[start_of_slice...end_of_slice]
    //
    //
    // TODO: refactor this so that we render in one iteration
    //
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
            let mut style = Style::default();

            let content: String = if database.is_focused {
                current_focused_index_position = i;
                style = style.bg(Color::Blue);
                String::from(">>>") + &database.name
            } else {
                database.name.to_string()
            };

            lines_to_draw.push(BufferLine {
                x,
                y,
                content,
                width: inner_area.width as usize,
                style,
            });

            if database.is_connected {
                for (j, table) in database.tables.iter().enumerate() {
                    y += 1;

                    let mut style = Style::default();

                    let content: String = if table.is_focused {
                        current_focused_index_position = i + j + 1;
                        style = style.bg(Color::Blue);
                        String::from(">>>") + &table.name
                    } else {
                        table.name.to_string()
                    };

                    lines_to_draw.push(BufferLine {
                        x: x + 3,
                        y,
                        content,
                        width: inner_area.width as usize,
                        style,
                    });
                }
            }
            y += 1;
        }

        let total_lines = lines_to_draw.len();
        let height_of_tree = (inner_area.height - inner_area.y) as usize;
        let radius_of_tree = height_of_tree as usize / 2;
        let y_current = current_focused_index_position + inner_area.y as usize;

        if total_lines > height_of_tree && y_current > radius_of_tree {
            let offset = y_current - radius_of_tree;
            let start_of_slice = max(0, offset);
            let end_of_slice = min(total_lines, height_of_tree + offset);
            let offset_lines = &lines_to_draw[start_of_slice..end_of_slice];

            for line in offset_lines {
                buf.set_stringn(
                    line.x,
                    line.y - offset as u16,
                    &line.content,
                    line.width,
                    line.style,
                );
            }
        } else {
            let end_of_slice = min(total_lines, height_of_tree);
            let offset_lines = &lines_to_draw[0..end_of_slice];
            for line in offset_lines {
                buf.set_stringn(line.x, line.y, &line.content, line.width, line.style);
            }
        }
    }
}
