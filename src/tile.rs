use std::ops::{Deref, DerefMut, Shl, Shr};

#[derive(Debug, Eq, PartialEq)]
pub struct Tile {
    palette_indexes: [u8; 64] // Linear
}

impl Default for Tile {
    fn default() -> Self {
        Tile::new([0; 64])
    }
}

impl Tile {
    pub fn new(palette_indexes: [u8; 64]) -> Self {
        Self { palette_indexes }
    }
}

impl Deref for Tile {
    type Target = [u8; 64];

    fn deref(&self) -> &Self::Target {
        &self.palette_indexes
    }
}

impl DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.palette_indexes
    }
}

impl Into<[u8; 32]> for &Tile {
    fn into(self) -> [u8; 32] {
        self.palette_indexes.chunks_exact(2)
            .map(|chunk| (chunk[0] & 0xf) | (chunk[1] & 0xf).shl(4))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }
}

impl From<[u8; 32]> for Tile {
    fn from(value: [u8; 32]) -> Self {
        let palette_indexes = value.into_iter()
            .map(|ns| [ns & 0xf, (ns & 0xf0).shr(4)])
            .flatten()
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        Self { palette_indexes }
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::Tile;

    #[test]
    fn tile_round_trip() {
        let mut palette_indexes: [u8; 64] = [ 0u8; 64 ];
        for it in 1..=8 {
            palette_indexes[it] = it as u8;
        }

        let tile = Tile::new(palette_indexes);
        let bytes: [u8; 32] = (&tile).into();

        assert_eq!(&bytes[..5], &[0x10, 0x32, 0x54, 0x76, 0x08]);
        assert_eq!(&bytes[5..], &[0u8; 27]);

        let tile = Tile::from(bytes);

        assert_eq!(&tile[..9], &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(&tile[9..], &[0u8; 55]);
    }
}