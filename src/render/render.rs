use crate::map::TileMap;
use crate::palette::Palette;
use crate::render::image::{BORDER_WIDTH, ImageData, OpaqueImageData, TransparencyImageData};
use crate::screen::Screen;
use crate::tile::Tile;

pub const DIM_CURSOR: usize = 8;

pub fn from_dimensions((width, height): &(usize, usize), map: impl Fn(usize) -> u8) -> Vec<u8> {
    (0usize..width * height).map(map).collect::<Vec<u8>>()
}

pub fn render_palette(palette: &Palette) -> impl ImageData {
    let dimensions = (16, 16);
    let data = from_dimensions(&dimensions, |idx| {
        if idx < palette.len() { idx as u8 } else { 0u8 }
    });
    OpaqueImageData::<'_> {
        palette,
        data,
        dimensions,
    }
}

pub fn render_tiles(palette: &Palette, tile_map: &TileMap) -> Vec<impl ImageData> {
    tile_map
        .iter()
        .map(|tile| render_tile(palette, tile))
        .collect()
}

pub fn scaled_palette_index(
    factor: usize,
    pixel_index: usize,
    (width, height): &(usize, usize),
) -> usize {
    let row = pixel_index / (height * factor);
    let column = (pixel_index % width) / factor;
    row * factor + column
}

pub fn render_tile(palette: &Palette, tile: &Tile) -> impl ImageData {
    let dimensions = (8, 8);
    let data = tile.to_vec();
    OpaqueImageData::<'_> {
        palette,
        data,
        dimensions,
    }
}

pub fn render_screen(
    palette: &Palette,
    character_data: &TileMap,
    screen_data: &Screen,
) -> impl ImageData {
    let mut data = vec![0u8; 240 * 160];

    for y in 0..20 {
        for x in 0..30 {
            let character = screen_data.get_character(x, y);

            let tile = character_data.get(character.tile_number()).unwrap();
            let tile = flip_tile(tile, character.vertical_flip(), character.horizontal_flip());

            for tile_y in 0..8 {
                for tile_x in 0..8 {
                    let pixel_data_index = tile_y * 8 + tile_x;
                    let palette_index = tile[pixel_data_index];

                    let x_index = x * 8 + tile_x;
                    let y_index = y * 8 + tile_y;
                    let data_index = y_index * 240 + x_index;
                    data[data_index] = palette_index;
                }
            }
        }
    }

    TransparencyImageData {
        opaque: OpaqueImageData::<'_> {
            palette,
            data,
            dimensions: (240, 160),
        },
    }
}

fn flip_tile(tile: &Tile, vflip: bool, hflip: bool) -> Tile {
    let transform = |(row, col): (u8, u8)| {
        let r = if vflip { 7 - row } else { row };
        let c = if hflip { 7 - col } else { col };
        r * 8 + c
    };

    Tile::new(std::array::from_fn::<u8, 64, _>(|i| {
        tile[transform((i as u8 / 8, i as u8 % 8)) as usize]
    }))
}

fn render_background(palette: &Palette, (width, height): (usize, usize)) -> OpaqueImageData<'_> {
    OpaqueImageData {
        palette,
        data: vec![0; width * height],
        dimensions: (width, height),
    }
}

pub fn render_cursor(
    palette: &Palette,
    (width, height): (usize, usize),
    cursor_x: usize,
    cursor_y: usize,
) -> TransparencyImageData<'_> {
    let mut data = vec![0; width * height];
    let cursor_side = DIM_CURSOR + 2 * BORDER_WIDTH;
    let row = cursor_y * DIM_CURSOR;
    let col = cursor_x * DIM_CURSOR;
    for idy in 0..cursor_side {
        for idx in 0..cursor_side {
            let is_border = idx < BORDER_WIDTH
                || idx >= cursor_side - BORDER_WIDTH
                || idy < BORDER_WIDTH
                || idy >= cursor_side - BORDER_WIDTH;
            if is_border {
                data[(row + idy) * width + col + idx] = 1;
            }
        }
    }
    TransparencyImageData {
        opaque: OpaqueImageData {
            palette,
            data,
            dimensions: (width, height),
        },
    }
}
