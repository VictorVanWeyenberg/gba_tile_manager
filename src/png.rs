use crate::map::TileMap;
use crate::palette::Palette;
use crate::render;
use crate::render::ImageData;
use crate::screen::Screen;
use crate::tile::Tile;
use std::io::Write;

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

pub fn screen_to_png(palette: &Palette, tile_map: &TileMap, screen: &Screen) -> Vec<u8> {
    let mut writer = vec![];
    image_data_to_png(render::render_screen(palette, tile_map, screen), &mut writer);
    writer
}

fn image_data_to_png(image_data: impl ImageData, writer: impl Write) {
    let palette: Vec<u8> = image_data.palette()
        .into_iter()
        .map(|color| color.as_png_rgb())
        .flatten()
        .collect();
    let data: Vec<u8> = image_data.data()
        .chunks_exact(2)
        .map(|idx| (idx[0] << 4) | idx[1])
        .collect();

    let (width, height) = image_data.dimensions();
    let mut encoder = png::Encoder::new(writer, *width, *height);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Four);
    encoder.set_palette(palette);
    encoder.set_trns(image_data.trns());
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}
