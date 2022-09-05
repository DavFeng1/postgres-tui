use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    style::{ Color, Modifier, Style},
    text::Span,
    Frame,
};


// Render main view
pub fn render_main_view<B: Backend>(f: &mut Frame<B>) {

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ]
            .as_ref(),
        )
        .horizontal_margin(1)
        .vertical_margin(1)
        .split(f.size());

    let default_style = Style::default().fg(Color::Red);

    let block = Block::default()
        .title(" Explorer ")
        .borders(Borders::ALL)
        .style(default_style);

    f.render_widget(block, chunks[0]);

    let block = Block::default()
        .title(" Main View ")
        .borders(Borders::ALL)
        .style(default_style);

    f.render_widget(block, chunks[1]);
}



pub fn render_topbar<B: Backend>(f: &mut Frame<B>, app: &App) {

    let size = f.size();

    let text = if app.show_popup {
        "Press p to close the popup, press q to quit"
    } else {
        "Press p to show the popup, press q to quit"
    };

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(size);

    let paragraph = Paragraph::new(
        Span::styled(
            text,
            Style::default().add_modifier(Modifier::SLOW_BLINK),
        ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunks[0]);
}

// Testing popups
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    render_topbar(f, app);

    render_main_view(f);

    if app.show_popup {

        let block = Block::default().title("Popup").borders(Borders::ALL);
        let area = centered_rect(60, 20, size);

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);

    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
