use std::fmt::Debug;
use std::path::PathBuf;
use image::{DynamicImage, ImageReader};
use crate::character_data::CharacterData;
use crate::error::Error;
use crate::palette::Palette;
use crate::project::{colors_to_palette_index, tiles_from_pal_idx, tiles_to_characters};
use crate::project::digest::Digests;
use crate::screen::ScreenData;

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
            image: ImageReader::open(&path)?.decode()?,
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