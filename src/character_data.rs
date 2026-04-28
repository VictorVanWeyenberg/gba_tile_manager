use crate::savable::Savable;
use crate::tile::Tile;
use std::ops::Deref;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CharacterData {
    name: String,
    tiles: Vec<Tile>,
}

impl CharacterData {
    pub fn with_tiles(name: impl ToString, mut tiles: Vec<Tile>) -> Self {
        while let Some(Tile { palette_indexes }) = tiles.last() {
            if palette_indexes.iter().any(|&p| p != 0) {
                break;
            }
            tiles.pop();
        }
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

    fn as_data(&self) -> Vec<u8> {
        self.tiles
            .iter()
            .map::<[u8; 32], _>(|tile| tile.into())
            .flatten()
            .collect()
    }
}
