use crate::color::Color;
use crate::savable::Savable;
use std::ops::Deref;

#[derive(Debug, Eq, PartialEq)]
pub struct Palette {
    name: String,
    colors: Vec<Color>,
}

impl Palette {
    pub fn with_colors(name: impl ToString, mut colors: Vec<Color>) -> Self {
        while let Some(Color { r: 0, g: 0, b: 0 }) = colors.last() {
            colors.pop();
        }
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

    fn as_data(&self) -> Vec<u8> {
        self.iter()
            .flat_map(|c| -> [u8; 2] { c.into() })
            .collect()
    }
}
