use crate::color::Color;
use crate::project::{Project, VRamData};
use crate::render;
use crate::render::ImageData;
use std::io::Write;
use crate::palette::Palette;
use crate::tile::Tile;

pub fn palette_to_png(palette: &Palette) -> Vec<u8> {
    let mut writer = vec![];
    image_data_to_png(render::render_palette(palette), &mut writer);
    writer
}

pub fn tile_to_png(palette: &Palette, tile: &Tile) -> Vec<u8> {
    let mut writer = vec![];
    image_data_to_png(render::render_tile(palette, tile), &mut writer);
    writer
}

pub fn screen_to_png(palette: &Palette, vram_data: &VRamData) -> Vec<u8> {
    let mut writer = vec![];
    image_data_to_png(render::render_screen(palette, vram_data), &mut writer);
    writer
}

fn image_data_to_png(ImageData { palette, data, dimensions: (width, height) }: ImageData, writer: impl Write) {
    let palette: Vec<u8> = palette
        .into_iter()
        .map(Color::as_png_rgb)
        .flatten()
        .collect();
    let data: Vec<u8> = data
        .chunks_exact(2)
        .map(|idx| (idx[0] << 4) | idx[1])
        .collect();

    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Four);
    encoder.set_palette(palette);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}
