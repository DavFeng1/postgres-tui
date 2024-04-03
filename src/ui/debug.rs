use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::Component;

pub struct DebugPopup {
    percent_x: u16,
    percent_y: u16,
    message: String,
}

impl DebugPopup {
    pub fn new(percent_x: u16, percent_y: u16, message: String) -> DebugPopup {
        DebugPopup {
            percent_x,
            percent_y,
            message,
        }
    }
}

impl Component for DebugPopup {
    fn render(&self, f: &mut Frame) {
        let size = f.size();

        let block = Block::default().borders(Borders::ALL).title("Debug");

        let input = Paragraph::new(self.message.clone())
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left)
            .block(block);

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - self.percent_y) / 2),
                    Constraint::Percentage(self.percent_y),
                    Constraint::Percentage((100 - self.percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(size);

        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - self.percent_x) / 2),
                    Constraint::Percentage(self.percent_x),
                    Constraint::Percentage((100 - self.percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(vertical_layout[1])[1];

        f.render_widget(Clear, area);
        f.render_widget(input, area);
    }
}
