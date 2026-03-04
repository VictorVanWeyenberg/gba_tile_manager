use crate::color::Color;
use std::io::Read;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }
}

impl Deref for Palette {
    type Target = Vec<Color>;

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

impl DerefMut for Palette {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.colors
    }
}

impl<T> From<T> for Palette
where
    T: Read,
{
    fn from(mut value: T) -> Self {
        let mut buf = [0u8; 2];
        let mut colors = vec![];
        while value.read_exact(&mut buf).is_ok() {
            colors.push(Color::from(buf));
        }
        Palette { colors }
    }
}

impl Into<Vec<u8>> for &Palette {
    fn into(self) -> Vec<u8> {
        self.colors.iter()
            .map(|c| -> [u8; 2] { c.into() })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use crate::color::Color;
    use crate::palette::Palette;

    #[test]
    fn palette_round_trip() {
        let mut palette = Palette::default();
        palette.push(Color::new(31, 0, 0).unwrap());
        palette.push(Color::new(0, 31, 0).unwrap());
        palette.push(Color::new(0, 0, 31).unwrap());
        palette.push(Color::new(31, 31, 31).unwrap());
        palette.push(Color::new(0, 0, 0).unwrap());

        let bytes: Vec<u8> = (&palette).into();
        assert_eq!(bytes, vec![0x1f, 0x00, 0xe0, 0x03, 0x00, 0x7c, 0xff, 0x7f, 0x00, 0x00]);

        let read = BufReader::new(&bytes as &[u8]);
        let mut palette = Palette::from(read);

        assert_eq!(palette.len(), 5);
        assert_eq!(palette.remove(0), Color::new(31, 0, 0).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 31, 0).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 0, 31).unwrap());
        assert_eq!(palette.remove(0), Color::new(31, 31, 31).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 0, 0).unwrap());
    }
}

