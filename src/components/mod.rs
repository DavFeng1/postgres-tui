pub mod popup;
pub mod topbar;
pub mod sidebar;
pub mod datatable;

use std::io;
use tui::{
    backend::CrosstermBackend,
    Frame,
};

use crate::app::App;

pub trait Component {
    fn render(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>);
}

pub fn draw(f: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut App) {
    topbar::render(f, app);
    sidebar::render(f, app);
    datatable::render(f, app);

    if app.show_keybinds {
        let p = popup::KeybindsPopup::new(60, 40);
        p.render(f);
    }
}


