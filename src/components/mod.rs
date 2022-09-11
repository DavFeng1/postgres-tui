pub mod popup;
pub mod searchbar;
pub mod sidebar;
pub mod datatable;
pub mod statusline;

use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Direction},
    Frame,
};

use crate::app::App;

pub trait Component {
    fn render(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>);
}

pub fn draw(f: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut App) {

    let area = f.size();

    let horizontal_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref()
        )
        .split(area);

    let statusline_area = Layout::default()
        .constraints(
            [
                Constraint::Ratio(23, 24),
                Constraint::Ratio(1, 24),
            ].as_ref()
        )
        .split(area)[1];

    let left_vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(23, 24),
                Constraint::Ratio(1, 24),

            ].as_ref()
        )
        .split(horizontal_split[0]);

    let right_vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(2, 24),
                Constraint::Ratio(21,24),
                Constraint::Ratio(1,24),

            ].as_ref()
        )
        .split(horizontal_split[1]);

    sidebar::render(f, app, left_vertical_split[0]);
    datatable::render(f, app, right_vertical_split[1]);
    searchbar::render(f, app, right_vertical_split[0]);
    statusline::render(f, app, statusline_area);

    if app.show_keybinds {
        let p = popup::KeybindsPopup::new(60, 40);
        p.render(f);
    }
}


