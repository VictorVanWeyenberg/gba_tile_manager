use crate::color::Color;
use crate::savable::Savable;
use std::io::Read;
use std::ops::{Deref, DerefMut};

const CURSOR_PALETTE_NAME: &str = "CURSOR_PALETTE";

#[derive(Debug, Eq, PartialEq)]
pub enum Palette {
    Cursor,
    Gba { name: String, colors: Vec<Color> },
}

impl Palette {
    pub fn new(name: impl ToString) -> Self {
        Palette::with_colors(name, vec![])
    }

    pub fn with_colors(name: impl ToString, colors: Vec<Color>) -> Self {
        Self::Gba {
            name: name.to_string(),
            colors,
        }
    }
}

impl Deref for Palette {
    type Target = Vec<Color>;

    fn deref(&self) -> &Self::Target {
        if let Self::Gba { colors, .. } = self {
            colors
        } else {
            panic!("Dereferencing a static palette.")
        }
    }
}

impl DerefMut for Palette {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Self::Gba { colors, .. } = self {
            colors
        } else {
            panic!("Dereferencing a static palette.")
        }
    }
}

impl Savable for Palette {
    fn name(&self) -> &str {
        if let Self::Gba { name, .. } = self {
            name
        } else {
            CURSOR_PALETTE_NAME
        }
    }

    fn suffix() -> &'static str {
        "_palette.bin"
    }

    fn create<R: Read>(name: impl ToString, mut data: R) -> Self {
        let mut buf = [0u8; 2];
        let mut colors = vec![];
        while data.read_exact(&mut buf).is_ok() {
            colors.push(Color::from(buf));
        }
        Palette::with_colors(name, colors)
    }

    fn as_data(&self) -> Vec<u8> {
        self.iter()
            .map(|c| -> [u8; 2] { c.into() })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::palette::Palette;
    use crate::savable::Savable;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn palette_round_trip() {
        let temp_dir = TempDir::new("gba_tile_manager::palette_round_trip")
            .unwrap()
            .path()
            .to_owned();
        fs::create_dir(temp_dir.clone()).unwrap();

        let mut palette = Palette::new("test");
        palette.push(Color::new(31, 0, 0).unwrap());
        palette.push(Color::new(0, 31, 0).unwrap());
        palette.push(Color::new(0, 0, 31).unwrap());
        palette.push(Color::new(31, 31, 31).unwrap());
        palette.push(Color::new(0, 0, 0).unwrap());

        let palette_path = palette.save(temp_dir).expect("Could not save palette.");
        let mut palette = Palette::read(palette_path).expect("Could not read palette.");

        assert_eq!(palette.len(), 5);
        assert_eq!(palette.remove(0), Color::new(31, 0, 0).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 31, 0).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 0, 31).unwrap());
        assert_eq!(palette.remove(0), Color::new(31, 31, 31).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 0, 0).unwrap());
    }
}
