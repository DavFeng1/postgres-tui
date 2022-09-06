use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    widgets::{Paragraph, Wrap},
    style::{Modifier, Style},
    text::Span,
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {

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

