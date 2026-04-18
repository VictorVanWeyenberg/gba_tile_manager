use std::fs::File;
use crate::savable::Savable;
use crate::tile::Tile;
use std::io::{BufReader, Read};
use std::ops::Deref;
use crate::error::Error;
use crate::palette::Palette;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CharacterData {
    name: String,
    tiles: Vec<Tile>,
}

impl CharacterData {
    pub fn with_tiles(name: impl ToString, tiles: Vec<Tile>) -> Self {
        Self {
            name: name.to_string(),
            tiles,
        }
    }
}

impl Deref for CharacterData {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl Savable for CharacterData {
    fn name(&self) -> &str {
        &self.name
    }

    fn suffix() -> &'static str {
        "_character_data.bin"
    }

    fn create<R: Read>(name: impl ToString, mut data: R) -> Self {
        let mut buf = [0u8; 32];
        let mut tiles = vec![];
        while data.read_exact(&mut buf).is_ok() {
            tiles.push(Tile::from(buf));
        }
        CharacterData {
            name: name.to_string(),
            tiles,
        }
    }

    fn as_data(&self) -> Vec<u8> {
        self.tiles
            .iter()
            .map::<[u8; 32], _>(|tile| tile.into())
            .flatten()
            .collect()
    }
}

impl TryFrom<&png::Reader<BufReader<File>>> for CharacterData {
    type Error = Error;

    fn try_from(reader: &png::Reader<BufReader<File>>) -> Result<Self, Self::Error> {
        todo!("Character data from PNG.")
    }
}
