use crate::color::Color;
use crate::project::Savable;
use crate::render::{from_dimensions, ImageData, ToHandle};
use iced::widget::image::Handle;
use std::io::Read;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq)]
pub struct Palette {
    name: String,
    colors: Vec<Color>,
}

impl Palette {
    pub fn new(name: impl ToString) -> Self {
        Palette::with_colors(name, vec![])
    }

    pub fn with_colors(name: impl ToString, colors: Vec<Color>) -> Self {
        Self { name: name.to_string(), colors }
    }

    pub fn set_color(&mut self, index: usize, color: Color) {
        while index >= self.colors.len() {
            self.colors.push(Color::black())
        }

        self.colors[index] = color
    }

    pub fn render_square(&self) -> Handle {
        self.render_with_dimensions((16, 16))
    }

    pub fn render_colors(&self) -> Vec<Handle> {
        (0..self.colors.len())
            .map(|idx| {
                ImageData::<'_> {
                    palette: self,
                    data: vec![idx as u8],
                    dimensions: (1, 1),
                    transparent: false,
                }.to_handle()
            })
            .collect()
    }

    fn render_with_dimensions(&self, dimensions: (usize, usize)) -> Handle {
        let data = from_dimensions(&dimensions, |idx| {
            if idx < self.len() { idx as u8 } else { 0u8 }
        });
        ImageData::<'_> {
            palette: self,
            data,
            dimensions,
            transparent: false,
        }.to_handle()
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

impl Savable for Palette {
    fn name(&self) -> &String {
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
        self.colors.iter()
            .map(|c| -> [u8; 2] { c.into() })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::palette::Palette;
    use crate::project::Savable;
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

        let palette_path = (palette).save(temp_dir).expect("Could not save palette.");
        let mut palette = Palette::read(palette_path).expect("Could not read palette.");

        assert_eq!(palette.len(), 5);
        assert_eq!(palette.remove(0), Color::new(31, 0, 0).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 31, 0).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 0, 31).unwrap());
        assert_eq!(palette.remove(0), Color::new(31, 31, 31).unwrap());
        assert_eq!(palette.remove(0), Color::new(0, 0, 0).unwrap());
    }
}

