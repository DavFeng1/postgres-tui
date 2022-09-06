mod crossterm;
mod app;
mod ui;

mod components;

use crate::crossterm::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()
}


