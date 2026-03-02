use crate::tile::Tile;
use std::io::Read;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct TileMap {
    tiles: Vec<Tile>,
}

impl Deref for TileMap {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}

impl Into<Vec<u8>> for &TileMap {
    fn into(self) -> Vec<u8> {
        self.tiles.iter()
            .map::<[u8; 32], _>(|tile| tile.into())
            .flatten()
            .collect()
    }
}

impl<T> From<T> for TileMap
where
    T: Read,
{
    fn from(mut value: T) -> Self {
        let mut buf = [0u8; 32];
        let mut tiles = vec![];
        while value.read_exact(&mut buf).is_ok() {
            tiles.push(Tile::from(buf));
        }
        TileMap { tiles }
    }
}