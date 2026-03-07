use crate::color::Color;
use crate::project::{Project, VRamData};
use crate::render;
use crate::render::ImageData;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

pub fn project_to_pngs(project: &Project) {
    for (name, vram_data) in project.screens() {
        screen_to_png(project, name, vram_data);
    }
}

fn screen_to_png(project: &Project, name: &str, vram_data: &VRamData) {
    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push(format!("{}.png", name));
    let file = File::create(file).unwrap();
    let ref mut w = BufWriter::new(file);

    let ImageData { palette, data } = render::render_screen(project, vram_data);
    let palette: Vec<u8> = palette
        .into_iter()
        .map(Color::as_png_rgb)
        .flatten()
        .collect();
    let data: Vec<u8> = data
        .chunks_exact(2)
        .map(|idx| (idx[0] << 4) | idx[1])
        .collect();

    let mut encoder = png::Encoder::new(w, 240, 160);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Four);
    encoder.set_palette(palette);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}
