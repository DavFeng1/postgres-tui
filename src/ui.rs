use crate::app::{App, InputMode};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph, Wrap, List, ListState, ListItem},
    style::{ Color, Modifier, Style},
    text::Span,
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    render_topbar(f, app);

    render_main_view(f);

    if app.show_popup {
        render_popup(f, app, 60, 40)
    }
}

pub fn render_topbar<B: Backend>(f: &mut Frame<B>, app: &App) {

    let size = f.size();

    let text = if app.show_popup {

        format!("{:?} Mode: Press p to close the popup, press q to quit", app.input_mode)

    } else {

        format!("{:?} Mode: Press p to show the popup, press q to quit", app.input_mode)

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

    f.render_widget(paragraph, chunks[0])

}

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

fn render_popup<B: Backend>(f: &mut Frame<B>, app: &App, percent_x: u16, percent_y: u16) {

    let size = f.size();

    let popup_block = Block::default().borders(Borders::ALL).title(
        match app.input_mode {
            InputMode::Normal => "Normal mode",
            InputMode::Editing => "Editing mode"
        }
    );

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Green),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(popup_block);


    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(size);

    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(vertical_layout[1])[1];

    f.render_widget(Clear, area);
    f.render_widget(input, area);

    match app.input_mode {

        InputMode::Normal => {},

        InputMode::Editing => {
            f.set_cursor(
                area.x + app.input.chars().count() as u16 + 1,
                area.y + 1,
            )
        }
    }
}

