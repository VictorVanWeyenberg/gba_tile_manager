mod domain;
mod render;

use crate::color::Color;
use crate::palette::Palette;
use crate::project::Project;
use crate::render::Layers;
pub use domain::*;
use render::{render_palette, render_tile};
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("resources");
    let project = Project::try_from(directory).unwrap();

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/empty_art.png");

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/background_palette.png");

    let data = render_palette(project.background_palette())
        .scale(8)
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
    .to_png();
    fs::write(file, &*data).unwrap();

    let cursor_palette = Palette::new(vec![Color::black(), Color::new(0, 31, 31).unwrap()]);

    let layer_names = vec!["empty_art_background", "empty_art_bg0", "empty_art_bg1", "empty_art_cursor"];
    let layers = Layers::new_screen(project.background_palette(), project.screens().get("empty_art").unwrap())
        .set_cursor(&cursor_palette, 0, 0);
    for (name, layer) in layer_names.into_iter().zip(layers.to_pngs()) {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push(format!("resources/{}.png", name));
        fs::write(file, &*layer).unwrap();
    }

    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/empty_art.png");
    fs::write(file, &*layers.to_png()).unwrap();
}
