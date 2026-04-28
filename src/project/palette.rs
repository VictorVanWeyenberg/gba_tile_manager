use crate::color::Color;
use crate::error::Error;
use crate::palette::Palette;
use crate::project::character::CharacterNode;
use crate::project::digest::Digests;
use image::{DynamicImage, ImageReader};
use std::fmt::Debug;
use std::path::PathBuf;

pub struct PaletteNode {
    name: String,
    image: DynamicImage,
    character_maps: Vec<CharacterNode>,
}

impl Debug for PaletteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.name, self.character_maps)
    }
}

impl PaletteNode {
    pub fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        let image = ImageReader::open(&path)
            .map_err(|e| Error::IO(e, path.to_str().unwrap().to_string()))?
            .decode()?;
        Ok(Self {
            name,
            image,
            character_maps: vec![],
        })
    }

    pub fn verify(&self) -> Result<(), Error> {
        if self.image.width() != 16 || self.image.height() != 16 {
            return Err(Error::Custom(format!(
                "Palette dimensions off ({}x{}) != (16x16)",
                self.image.width(),
                self.image.height()
            )));
        }
        for character_map in &self.character_maps {
            character_map.verify()?;
        }
        Ok(())
    }

    pub fn digest(&mut self, digests: &mut Digests) -> Result<(), Error> {
        let palette = self.as_palette()?;
        for character_map in &mut self.character_maps {
            character_map.digest(digests, &palette)?;
        }
        digests.palettes_mut().push(palette);
        Ok(())
    }

    pub fn as_palette(&mut self) -> Result<Palette, Error> {
        let colors = self
            .image
            .to_rgb8()
            .chunks_exact(3)
            .map(|c| Color::new(c[0] / 8, c[1] / 8, c[2] / 8).unwrap())
            .collect();
        Ok(Palette::with_colors(&self.name, colors))
    }

    pub fn character_maps_mut(&mut self) -> &mut Vec<CharacterNode> {
        &mut self.character_maps
    }
}
