use crate::character::Character;
use crate::savable::Savable;
use std::ops::Deref;

#[derive(Debug, Eq, PartialEq)]
pub struct ScreenData {
    name: String,
    characters: Vec<Character>,
}

impl ScreenData {
    pub fn with_characters(name: &str, mut characters: Vec<Character>) -> Self {
        while let Some(Character {
            tile_number: 0,
            horizontal_flip: false,
            vertical_flip: false,
            palette_number: 0,
        }) = characters.last()
        {
            characters.pop();
        }
        Self {
            name: name.to_string(),
            characters,
        }
    }
}

impl Deref for ScreenData {
    type Target = Vec<Character>;
    fn deref(&self) -> &Self::Target {
        &self.characters
    }
}

impl Savable for ScreenData {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_data(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self
            .characters
            .iter()
            .flat_map(<&Character as Into<[u8; 2]>>::into)
            .collect();

        bytes
            .chunks_exact(2)
            .rposition(|b| b[0] != 0 || b[1] != 0)
            .map_or(&[][..], |i| &bytes[..=i * 2 + 1])
            .into()
    }
}
