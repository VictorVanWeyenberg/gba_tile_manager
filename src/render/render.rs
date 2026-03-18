use crate::color::Color;
use crate::map::TileMap;
use crate::palette::Palette;
use crate::render::ImageData;
use crate::screen::Screen;
use crate::tile::Tile;
use lazy_static::lazy_static;

pub const DIM_CURSOR: usize = 8;
lazy_static! {
    static ref CURSOR_PALETTE: Palette =
        Palette::new(vec![Color::black(), Color::new(0, 31, 31).unwrap(),]);
}

pub fn from_dimensions((width, height): &(usize, usize), map: impl Fn(usize) -> u8) -> Vec<u8> {
    (0usize..width * height).map(map).collect::<Vec<u8>>()
}

pub fn render_tiles<'c>(palette: &'c Palette, tile_map: &TileMap) -> Vec<ImageData<'c>> {
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

pub fn render_tile<'c>(palette: &'c Palette, tile: &Tile) -> ImageData<'c> {
    let dimensions = (8, 8);
    let data = tile.to_vec();
    ImageData::<'_> {
        palette,
        data,
        dimensions,
        transparent: false,
    }
}

pub fn render_screen<'c>(
    palette: &'c Palette,
    character_data: &TileMap,
    screen_data: &Screen,
) -> ImageData<'c> {
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

    ImageData {
        palette,
        data,
        dimensions: (240, 160),
        transparent: true,
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

pub fn render_background(palette: &Palette, (width, height): (usize, usize)) -> ImageData<'_> {
    ImageData {
        palette,
        data: vec![0; width * height],
        dimensions: (width, height),
        transparent: false,
    }
}

pub fn render_cursor<'c>(
    (width, height): (usize, usize),
    cursor_x: usize,
    cursor_y: usize,
) -> ImageData<'c> {
    let width = width * DIM_CURSOR;
    let height = height * DIM_CURSOR;
    let mut data = vec![0; width * height];
    let row = cursor_y * DIM_CURSOR;
    let col = cursor_x * DIM_CURSOR;
    for idy in 0..DIM_CURSOR {
        for idx in 0..DIM_CURSOR {
            let is_border = idy == 0 || idx == 0 || idx == DIM_CURSOR - 1 || idy == DIM_CURSOR - 1;
            if is_border {
                data[(row + idy) * width + col + idx] = 1;
            }
        }
    }
    ImageData {
        palette: &CURSOR_PALETTE,
        data,
        dimensions: (width, height),
        transparent: true,
    }
}
