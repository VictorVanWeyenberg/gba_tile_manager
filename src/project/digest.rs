use crate::boop::Boops;
use crate::character_data::CharacterData;
use crate::error::Error;
use crate::palette::Palette;
use crate::savable::Savable;
use crate::screen::ScreenData;
use std::fs;
use std::path::PathBuf;

#[derive(Default)]
pub struct Digests {
    palettes: Vec<Palette>,
    characters: Vec<CharacterData>,
    screens: Vec<ScreenData>,
    boops: Vec<Boops>,
}

impl Digests {
    pub fn save(&self, path: PathBuf, flatten: bool) -> Result<(), Error> {
        if !path.exists() {
            fs::create_dir(&path).map_err(|e| Error::IO(e, path.to_str().unwrap().to_string()))?;
        }
        for palette in &self.palettes {
            palette.save(path.clone(), flatten)?;
        }
        for character_data in &self.characters {
            character_data.save(path.clone(), flatten)?;
        }
        for screen_data in &self.screens {
            screen_data.save(path.clone(), flatten)?;
        }
        for boops in &self.boops {
            boops.save(path.clone(), flatten)?;
        }
        Ok(())
    }

    pub fn palettes_mut(&mut self) -> &mut Vec<Palette> {
        &mut self.palettes
    }

    pub fn characters_mut(&mut self) -> &mut Vec<CharacterData> {
        &mut self.characters
    }

    pub fn screens_mut(&mut self) -> &mut Vec<ScreenData> {
        &mut self.screens
    }

    pub fn boops_mut(&mut self) -> &mut Vec<Boops> {
        &mut self.boops
    }
}
