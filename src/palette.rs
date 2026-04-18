use std::fs::File;
use crate::color::Color;
use crate::savable::Savable;
use std::io::{BufReader, Read};
use std::ops::Deref;
use png;
use crate::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Palette {
    name: String,
    colors: Vec<Color>,
}

impl Palette {
    pub fn with_colors(name: impl ToString, colors: Vec<Color>) -> Self {
        Self {
            name: name.to_string(),
            colors,
        }
    }
}

impl Deref for Palette {
    type Target = Vec<Color>;

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

impl Savable for Palette {
    fn name(&self) -> &str {
        &self.name
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

impl TryFrom<&png::Reader<BufReader<File>>> for Palette {
    type Error = Error;

    fn try_from(reader: &png::Reader<BufReader<File>>) -> Result<Self, Self::Error> {
        todo!("Palette from PNG.")
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

        let mut colors = vec![];
        colors.push(Color::new(31, 0, 0).unwrap());
        colors.push(Color::new(0, 31, 0).unwrap());
        colors.push(Color::new(0, 0, 31).unwrap());
        colors.push(Color::new(31, 31, 31).unwrap());
        colors.push(Color::new(0, 0, 0).unwrap());

        let palette = Palette::with_colors("test", colors);
        let palette_path = palette.save(temp_dir).expect("Could not save palette.");
        let palette = Palette::read(palette_path).expect("Could not read palette.");

        assert_eq!(palette.len(), 5);
        assert_eq!(palette.get(0), Some(&Color::new(31, 0, 0).unwrap()));
        assert_eq!(palette.get(1), Some(&Color::new(0, 31, 0).unwrap()));
        assert_eq!(palette.get(2), Some(&Color::new(0, 0, 31).unwrap()));
        assert_eq!(palette.get(3), Some(&Color::new(31, 31, 31).unwrap()));
        assert_eq!(palette.get(4), Some(&Color::new(0, 0, 0).unwrap()));
    }
}
