use crate::color::Color;
use crate::project::{Project, VRamData};
use crate::tile::Tile;

/// For a 3x2 image, image data will have data [1, 2, 3, 4, 5, 6] that's supposed to be rendered as
/// follows.
///
/// ```
/// 1 2 3
/// 4 5 6
/// ```
pub struct ImageData<'c> {
    pub palette: Vec<&'c Color>,
    pub data: Vec<u8>,
    pub dimensions: (u32, u32),
}

pub fn render_screen<'c>(
    project: &'c Project,
    VRamData {
        bg0_character_data,
        bg0_screen_data,
        bg1_character_data,
        bg1_screen_data,
    }: &VRamData,
) -> ImageData<'c> {
    let background_palette = project.background_palette();
    let mut data = vec![0u8; 240 * 160];

    for y in 0..20 {
        for x in 0..30 {
            let bg0_character = bg0_screen_data.get_character(x, y);
            let bg1_character = bg1_screen_data.get_character(x, y);

            let bg0_tile = bg0_character_data.get(bg0_character.tile_number()).unwrap();
            let bg1_tile = bg1_character_data.get(bg1_character.tile_number()).unwrap();

            let bg0_tile = flip_tile(
                bg0_tile,
                bg0_character.vertical_flip(),
                bg0_character.horizontal_flip(),
            );
            let bg1_tile = flip_tile(
                bg1_tile,
                bg1_character.vertical_flip(),
                bg1_character.horizontal_flip(),
            );

            for tile_y in 0..8 {
                for tile_x in 0..8 {
                    let pixel_data_index = tile_y * 8 + tile_x;
                    let bg0_palette_index = bg0_tile[pixel_data_index];
                    let bg1_palette_index = bg1_tile[pixel_data_index];
                    let palette_index = if bg0_palette_index != 0 {
                        bg0_palette_index
                    } else if bg1_palette_index != 0 {
                        bg1_palette_index
                    } else {
                        0
                    };

                    let x_index = x * 8 + tile_x;
                    let y_index = y * 8 + tile_y;
                    data[y_index * 240 + x_index] = palette_index;
                }
            }
        }
    }

    ImageData {
        palette: background_palette.iter().collect(),
        data,
        dimensions: (240, 160),
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
