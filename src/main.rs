extern crate core;

mod domain;
mod render;
mod ui;

use crate::project::Project;
pub use domain::*;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application(boot_fn, ui::update, ui::view).run()
}

fn boot_fn() -> ui::State {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("resources");
    let project = Project::try_from(directory).unwrap();
    ui::State::new(project)
}