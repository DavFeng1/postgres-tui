use crate::app::App;
use crate::components::{topbar, popup, sidebar, datatable};
use tui::{
    backend::Backend,
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    topbar::render(f, app);
    sidebar::render(f, app);
    datatable::render(f, app);

    if app.show_popup {
        popup::render(f, app, 60, 40)
    }
}


