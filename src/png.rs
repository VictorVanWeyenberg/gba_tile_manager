use crate::color::Color;
use crate::project::{Project, VRamData};
use crate::tile::Tile;
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
    writer.write_image_data(&render_screen(project, vram_data)).unwrap();
}

fn render_screen(
    project: &Project,
    VRamData {
        bg0_character_data,
        bg0_screen_data,
        bg1_character_data,
        bg1_screen_data,
    }: &VRamData,
) -> Vec<u8> {
    let background_color = &project.background_palette()[0];
    let mut png_data = [background_color; 240 * 160];

    for y in 0..20 {
        for x in 0..30 {
            let bg0_character = bg0_screen_data.get_character(x, y);
            let bg1_character = bg1_screen_data.get_character(x, y);

            let bg0_tile = bg0_character_data.get(bg0_character.tile_number()).unwrap();
            let bg1_tile = bg1_character_data.get(bg1_character.tile_number()).unwrap();

            let bg0_tile_colors = render_tile(project, bg0_tile, bg0_character.vertical_flip(), bg0_character.horizontal_flip());
            let bg1_tile_colors = render_tile(project, bg1_tile, bg1_character.vertical_flip(), bg1_character.horizontal_flip());

            for tile_y in 0..8 {
                for tile_x in 0..8 {
                    let pixel_data_index = tile_y * 8 + tile_x;
                    let bg0_color = bg0_tile_colors[pixel_data_index];
                    let bg1_color = bg1_tile_colors[pixel_data_index];
                    let color = pick_color(bg0_color, bg1_color, background_color);

                    let x_index = x * 8 + tile_x;
                    let y_index = y * 8 + tile_y;
                    png_data[y_index * 240 + x_index] = color;
                }
            }
        }
    }

    png_data.into_iter()
        .map(Color::as_png_rgba)
        .flatten()
        .collect()
}

fn render_tile<'c>(project: &'c Project, tile: &Tile, vflip: bool, hflip: bool) -> Vec<&'c Color> {
    let colors: Vec<&Color> = tile.iter()
        .map::<&Color, _>(|palette_index| &project.background_palette()[*palette_index as usize])
        .collect();

    let transform = |(row, col): (usize, usize)| {
        let r = if vflip { 7 - row } else { row };
        let c = if hflip { 7 - col } else { col };
        r * 8 + c
    };

    Vec::from(std::array::from_fn::<&Color, 64, _>(|i| colors[transform((i / 8, i % 8))]))
}

fn pick_color<'p>(
    bg0_pixel_data: &'p Color,
    bg1_pixel_data: &'p Color,
    background_color: &'p Color,
) -> &'p Color {
    if bg0_pixel_data != background_color {
        bg0_pixel_data
    } else if bg1_pixel_data != background_color {
        bg1_pixel_data
    } else {
        background_color
    }
}
