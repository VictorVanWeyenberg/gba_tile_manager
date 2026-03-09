mod domain;
mod png;
mod render;

use crate::png::{palette_to_png, screen_to_png, tile_to_png};
use crate::project::Project;
pub use domain::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("resources");
    let project = Project::try_from(directory).unwrap();

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/empty_art.png");

    let data = screen_to_png(project.background_palette(), project.screens().get("empty_art").unwrap());
    fs::write(file, &data).unwrap();

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/background_palette.png");

    let data = palette_to_png(project.background_palette());
    fs::write(file, &data).unwrap();

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/tile.png");

    let data = tile_to_png(project.background_palette(), project.screens().get("empty_art").unwrap().bg1_character_data.get(7).unwrap());
    fs::write(file, &data).unwrap();

}
