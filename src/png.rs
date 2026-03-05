use crate::project::{Project, VRamData};
use crate::render;
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

    let mut encoder = png::Encoder::new(w, 240, 160);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let data = render::render_screen(project, vram_data).into_iter()
        .map(|color| color.as_png_rgba())
        .flatten()
        .collect::<Vec<u8>>();
    writer.write_image_data(&data).unwrap();
}