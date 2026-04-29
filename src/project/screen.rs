use std::fmt::Debug;
use std::path::PathBuf;
use image::{DynamicImage, ImageReader};
use crate::character::Character;
use crate::character_data::CharacterData;
use crate::error::Error;
use crate::palette::Palette;
use crate::project::{colors_to_palette_index, tiles_from_pal_idx};
use crate::project::digest::Digests;
use crate::savable::Savable;
use crate::screen::ScreenData;
use crate::tile::Tile;

pub struct ScreenNode {
    name: String,
    image: DynamicImage,
}

impl Debug for ScreenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ScreenNode {
    pub fn new(name: impl ToString, path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            name: name.to_string(),
            image: ImageReader::open(&path)
                .map_err(|e| Error::IO(e, path.to_str().unwrap().to_string()))?
                .decode()?,
        })
    }

    pub fn verify(&self) -> Result<(), Error> {
        if self.image.width() != 256 || self.image.height() != 256 {
            return Err(Error::Custom(format!(
                "Screen data dimensions off ({}x{}) != (256x256)",
                self.image.width(),
                self.image.height()
            )));
        }
        Ok(())
    }

    pub fn as_screen_data(
        &mut self,
        palette: &Palette,
        character_data: &CharacterData,
    ) -> Result<ScreenData, Error> {
        let buf = self.image.to_rgb8().into_raw();
        let pal_idx = colors_to_palette_index(buf, palette)?;
        let tiles = tiles_from_pal_idx(pal_idx);
        let characters = tiles
            .into_iter()
            .map(|tile| tiles_to_characters(tile, character_data))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(ScreenData::with_characters(&self.name, characters))
    }

    pub fn digest(
        &mut self,
        digests: &mut Digests,
        palette: &Palette,
        character_data: &CharacterData,
    ) -> Result<(), Error> {
        Ok(digests
            .screens_mut()
            .push(self.as_screen_data(palette, character_data)?))
    }
}

fn tiles_to_characters(needle: Tile, haystack: &CharacterData) -> Result<Character, Error> {
    for (idx, tile) in haystack.iter().enumerate() {
        if &needle == tile {
            return Ok(Character::new(idx, false, false, 0));
        }
        if &flip_tile(&needle, true, false) == tile {
            return Ok(Character::new(idx, true, false, 0));
        }
        if &flip_tile(&needle, false, true) == tile {
            return Ok(Character::new(idx, false, true, 0));
        }
        if &flip_tile(&needle, true, true) == tile {
            return Ok(Character::new(idx, true, true, 0));
        }
    }
    let text_tile = needle
        .chunks_exact(8)
        .map(|chunk| format!("{chunk:?}"))
        .reduce(|a, b| format!("{a:?}\n{b:?}"))
        .unwrap_or_else(|| String::from(""));
    let character_data_name = haystack.name();
    Err(Error::Custom(format!(
        "Tile not found in character data {character_data_name}\
        \
        {text_tile}"
    )))
}

fn flip_tile(tile: &Tile, hflip: bool, vflip: bool) -> Tile {
    let mut result = [0u8; 64];
    for y in 0..8 {
        for x in 0..8 {
            let src_x = if hflip { 7 - x } else { x };
            let src_y = if vflip { 7 - y } else { y };
            result[y * 8 + x] = tile[src_y * 8 + src_x];
        }
    }
    Tile::new(result)
}