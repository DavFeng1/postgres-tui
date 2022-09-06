mod crossterm;
mod app;
mod ui;
mod postgres;

mod components;

use crate::crossterm::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()
}


