use crate::project::Savable;
use crate::tile::Tile;
use std::io::Read;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CharacterData {
    name: String,
    tiles: Vec<Tile>,
}

impl CharacterData {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            tiles: vec![],
        }
    }
}

impl Deref for CharacterData {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl DerefMut for CharacterData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}

impl Savable for CharacterData {
    fn name(&self) -> &String {
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
