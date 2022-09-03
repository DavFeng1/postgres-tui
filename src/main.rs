mod ui;
mod crossterm;
mod app;

use crate::crossterm::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()
}

