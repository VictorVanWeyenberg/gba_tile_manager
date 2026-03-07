mod domain;
mod png;
mod render;

use crate::png::screen_to_png;
use crate::project::Project;
pub use domain::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

fn main() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("resources");
    let project = Project::try_from(directory).unwrap();

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/empty_art.png");
    let file = File::create(file).unwrap();
    let ref mut writer = BufWriter::new(file);

    screen_to_png(&project, project.screens().get("empty_art").unwrap(), writer);
}
