use crate::character_data::CharacterData;
use crate::error::Error;
use crate::palette::Palette;
use crate::project::digest::Digests;
use crate::project::screen::ScreenNode;
use crate::project::{colors_to_palette_index, tiles_from_pal_idx};
use image::{DynamicImage, ImageReader};
use std::fmt::Debug;
use std::path::PathBuf;

pub struct CharacterNode {
    name: String,
    image: DynamicImage,
    screens: Vec<ScreenNode>,
}

impl Debug for CharacterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.name, self.screens)
    }
}

impl CharacterNode {
    pub fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        let image = ImageReader::open(&path)
            .map_err(|e| Error::IO(e, path.to_str().unwrap().to_string()))?
            .decode()?;
        Ok(Self {
            name,
            image,
            screens: vec![],
        })
    }

    pub fn verify(&self) -> Result<(), Error> {
        if self.image.width() != 256 || self.image.height() != 256 {
            return Err(Error::Custom(format!(
                "Character data dimensions off ({}x{}) != (256x256)",
                self.image.width(),
                self.image.height()
            )));
        }
        for screen in &self.screens {
            screen.verify()?
        }
        Ok(())
    }

    pub fn digest(&mut self, digests: &mut Digests, palette: &Palette) -> Result<(), Error> {
        let character_data = self.as_character_data(palette)?;
        for screen in &mut self.screens {
            screen.digest(digests, palette, &character_data)?;
        }
        digests.characters_mut().push(character_data);
        Ok(())
    }

    pub fn as_character_data(&mut self, palette: &Palette) -> Result<CharacterData, Error> {
        let buf = self.image.to_rgb8().into_raw();
        let pal_idx = colors_to_palette_index(buf, palette)?;
        let tiles = tiles_from_pal_idx(pal_idx);
        Ok(CharacterData::with_tiles(self.name.clone(), tiles))
    }

    pub fn screens_mut(&mut self) -> &mut Vec<ScreenNode> {
        &mut self.screens
    }
}
