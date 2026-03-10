mod domain;
mod render;

use crate::project::{Project, VRamData};
pub use domain::*;
use render::ImageData;
use render::{render_palette, render_screen, render_tile};
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("resources");
    let project = Project::try_from(directory).unwrap();

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/empty_art.png");

    let VRamData {
        bg0_character_data,
        bg0_screen_data,
        ..
    } = project.screens().get("empty_art").unwrap();
    let data = render_screen(
        project.background_palette(),
        bg0_character_data,
        bg0_screen_data,
    )
    .border(2)
    .to_png();
    fs::write(file, &*data).unwrap();

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/background_palette.png");

    let data = render_palette(project.background_palette())
        .scale(8)
        .border(2)
        .to_png();
    fs::write(file, &*data).unwrap();

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/tile.png");

    let data = render_tile(
        project.background_palette(),
        project
            .screens()
            .get("empty_art")
            .unwrap()
            .bg1_character_data
            .get(7)
            .unwrap(),
    )
    .scale(8)
    .border(2)
    .to_png();
    fs::write(file, &*data).unwrap();
}
