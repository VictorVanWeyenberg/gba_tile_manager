use crate::color::Color;
use crate::map::TileMap;
use crate::palette::Palette;
use crate::screen::Screen;
use crate::tile::Tile;
use std::borrow::Cow;

pub trait ImageData {
    fn palette(&self) -> &Vec<&Color>;
    fn data(&self) -> &Vec<u8>;
    fn trns(&self) -> impl Into<Cow<'_, [u8]>>;
    fn dimensions(&self) -> &(usize, usize);
    fn scale(self, factor: usize) -> Self;
}

/// For a 3x2 image, image data will have data [1, 2, 3, 4, 5, 6] that's supposed to be rendered as
/// follows.
///
/// ```
/// 1 2 3
/// 4 5 6
/// ```
struct OpaqueImageData<'c, const N: usize> {
    palette: Vec<&'c Color>,
    data: Vec<u8>,
    dimensions: (usize, usize),
}

impl<'c, const N: usize> ImageData for OpaqueImageData<'c, N> {
    fn palette(&self) -> &Vec<&Color> {
        &self.palette
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }

    fn trns(&self) -> impl Into<Cow<'_, [u8]>> {
        &[255; N]
    }

    fn dimensions(&self) -> &(usize, usize) {
        &self.dimensions
    }

    fn scale(self, factor: usize) -> Self {
        let Self {
            palette,
            data,
            dimensions: (width, height),
        } = self;
        let dimensions = (width * factor, height * factor);
        let data = from_dimensions(&dimensions, |idx| {
            let index = scaled_palette_index(factor, idx, &dimensions);
            data[index]
        });
        Self {
            palette,
            data,
            dimensions,
        }
    }
}

struct TransparencyImageData<'c, const N: usize> {
    opaque: OpaqueImageData<'c, N>,
    trns: Vec<u8>,
}

impl<'c, const N: usize> ImageData for TransparencyImageData<'c, N> {
    fn palette(&self) -> &Vec<&Color> {
        &self.opaque.palette
    }

    fn data(&self) -> &Vec<u8> {
        &self.opaque.data
    }

    fn trns(&self) -> impl Into<Cow<'_, [u8]>> {
        &self.trns
    }

    fn dimensions(&self) -> &(usize, usize) {
        &self.opaque.dimensions
    }

    fn scale(self, factor: usize) -> Self {
        let Self { opaque, trns } = self;
        let opaque = opaque.scale(factor);
        let trns = from_dimensions(opaque.dimensions(), |idx| {
            let index = scaled_palette_index(factor, idx, opaque.dimensions());
            trns[index]
        });
        Self { opaque, trns }
    }
}

fn from_dimensions((width, height): &(usize, usize), map: impl Fn(usize) -> u8) -> Vec<u8> {
    (0usize..width * height).map(map).collect::<Vec<u8>>()
}

pub fn render_palette(palette: &Palette) -> impl ImageData {
    let dimensions = (16, 16);
    let data = from_dimensions(&dimensions, |idx| {
        if idx < palette.len() {
            idx as u8
        } else {
            0u8
        }
    });
    OpaqueImageData::<'_, 256> {
        palette: palette.iter().collect(),
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

fn scaled_palette_index(
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
    OpaqueImageData::<'_, 64> {
        palette: palette.iter().collect(),
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
    let mut trns = vec![0u8; 240 * 160];

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
                    if palette_index != 0 {
                        trns[data_index] = 255
                    }
                }
            }
        }
    }

    TransparencyImageData {
        opaque: OpaqueImageData::<'_, 38400> {
            palette: palette.iter().collect(),
            data,
            dimensions: (240, 160),
        },
        trns,
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
