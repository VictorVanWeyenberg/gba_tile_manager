use std::fmt::{Display, Formatter};
use std::ops::{Shl, Shr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Result<Color, String> {
        match (r, g, b) {
            (r, g, b) if r > 31 || g > 31 || b > 31 => Err("Invalid color values".to_string()),
            _ => Ok(Color { r, g, b })
        }
    }
    
    pub fn black() -> Color {
        Color::new(0, 0, 0).unwrap()
    }

    pub fn as_png_rgb(&self) -> [u8; 3] {
        [self.r * 8, self.g * 8, self.b * 8]
    }

    pub fn as_rgba(&self, transparent: bool) -> [u8; 4] {
        let transparent = if transparent &&
            self.r == 0 &&
            self.g == 0 &&
            self.b == 0 {
            0
        } else {
            255
        };
        [self.r * 8, self.g * 8, self.b * 8, transparent]
    }
}

impl Into<[u8; 2]> for &Color {
    fn into(self) -> [u8; 2] {
        let color: u16 = (self.r as u16) | (self.g as u16).shl(5) | (self.b as u16).shl(10);
        color.to_le_bytes()
    }
}

impl From<[u8; 2]> for Color {
    fn from(value: [u8; 2]) -> Self {
        let value = u16::from_le_bytes(value);
        let r = (value & 0x001f) as u8;
        let g = (value & 0x03e0).shr(5) as u8;
        let b = (value & 0x7c00).shr(10) as u8;
        Color::new(r, g, b).unwrap()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn red_into() {
        let color = Color::new(31, 0, 0).unwrap();
        let bytes: [u8; 2] = (&color).into();
        assert_eq!(bytes, [0x1f, 0x00])
    }

    #[test]
    fn green_into() {
        let color = Color::new(0, 31, 0).unwrap();
        let bytes: [u8; 2] = (&color).into();
        assert_eq!(bytes, [0xe0, 0x03])
    }

    #[test]
    fn blue_into() {
        let color = Color::new(0, 0, 31).unwrap();
        let bytes: [u8; 2] = (&color).into();
        assert_eq!(bytes, [0x00, 0x7c])
    }

    #[test]
    fn white_into() {
        let color = Color::new(31, 31, 31).unwrap();
        let bytes: [u8; 2] = (&color).into();
        assert_eq!(bytes, [0xff, 0x7f])
    }

    #[test]
    fn black_into() {
        let color = Color::new(0, 0, 0).unwrap();
        let bytes: [u8; 2] = (&color).into();
        assert_eq!(bytes, [0x00, 0x00])
    }

    #[test]
    fn from_red() {
        let bytes = [0x1f, 0x00];
        let color = Color::from(bytes);
        assert_eq!(color.r, 31);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn from_green() {
        let bytes = [0xe0, 0x03];
        let color = Color::from(bytes);
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 31);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn from_blue() {
        let bytes = [0x00, 0x7c];
        let color = Color::from(bytes);
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 31);
    }

    #[test]
    fn from_white() {
        let bytes = [0xff, 0x7f];
        let color = Color::from(bytes);
        assert_eq!(color.r, 31);
        assert_eq!(color.g, 31);
        assert_eq!(color.b, 31);
    }

    #[test]
    fn from_black() {
        let bytes = [0x00, 0x00];
        let color = Color::from(bytes);
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }
}