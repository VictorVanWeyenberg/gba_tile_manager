use std::ops::{Shl, Shr};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Character {
    pub tile_number: usize,
    pub horizontal_flip: bool,
    pub vertical_flip: bool,
    pub palette_number: usize,
}

impl Character {
    pub fn new(tile_number: usize,
               horizontal_flip: bool,
               vertical_flip: bool,
               palette_number: usize,) -> Self {
        Character { tile_number, horizontal_flip, vertical_flip, palette_number }
    }

    pub fn tile_number(&self) -> usize {
        self.tile_number
    }

    pub fn horizontal_flip(&self) -> bool {
        self.horizontal_flip
    }

    pub fn vertical_flip(&self) -> bool {
        self.vertical_flip
    }

    pub fn palette_number(&self) -> usize {
        self.palette_number
    }

}

impl Into<[u8; 2]> for &Character {
    fn into(self) -> [u8; 2] {
        let bytes: u16 = (self.tile_number & 0x3ff) as u16 |
            (self.horizontal_flip as u16).shl(10) |
            (self.vertical_flip as u16).shl(11) |
            ((self.palette_number & 0xf) as u16).shl(12);
        bytes.to_le_bytes()
    }
}

impl From<[u8; 2]> for Character {
    fn from(value: [u8; 2]) -> Self {
        let value = u16::from_le_bytes(value);
        let tile_number = (value & 0x3ff) as usize;
        let horizontal_flip = value & 0x400 > 0;
        let vertical_flip = value & 0x800 > 0;
        let palette_number = (value & 0xf000).shr(12) as usize;
        Self { tile_number, horizontal_flip, vertical_flip, palette_number }
    }
}

#[cfg(test)]
mod tests {
    use crate::character::Character;

    #[test]
    fn text_round_trip() {
        let text = Character::new(1023, true, true, 15);
        let bytes: [u8; 2] = (&text).into();
        assert_eq!(bytes, [0xff, 0xff]);

        let text = Character::new(0, false, false, 0);
        let bytes: [u8; 2] = (&text).into();
        assert_eq!(bytes, [0x00, 0x00]);

        let text = Character::new(16, false, false, 0);
        let bytes: [u8; 2] = (&text).into();
        assert_eq!(bytes, [0x10, 0x00]);

        let text = Character::new(0, true, false, 0);
        let bytes: [u8; 2] = (&text).into();
        assert_eq!(bytes, [0x00, 0x04]);

        let text = Character::new(0, false, true, 0);
        let bytes: [u8; 2] = (&text).into();
        assert_eq!(bytes, [0x00, 0x08]);

        let text = Character::new(0, false, false, 7);
        let bytes: [u8; 2] = (&text).into();
        assert_eq!(bytes, [0x00, 0x70]);
    }
}

