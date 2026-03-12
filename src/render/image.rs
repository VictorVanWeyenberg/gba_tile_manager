use crate::palette::Palette;
use crate::render::png::{image_data_to_png, Png};
use crate::render::render::{from_dimensions, scaled_palette_index};
use crate::render::Layers;

pub const BORDER_WIDTH: usize = 2;

/// For a 3x2 image, image data will have data [1, 2, 3, 4, 5, 6] that's supposed to be rendered as
/// follows.
///
/// ```
/// 1 2 3
/// 4 5 6
/// ```
pub struct ImageData<'c> {
    pub palette: &'c Palette,
    pub data: Vec<u8>,
    pub dimensions: (usize, usize),
    pub transparent: bool,
}

impl<'c> ImageData<'c> {
    pub fn palette(&self) -> &'c Palette {
        &self.palette
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn trns(&self) -> Vec<u8> {
        let mut trns = vec![255; self.palette.len()];
        if self.transparent {
            trns[0] = 0;
        }
        trns
    }

    pub fn dimensions(&self) -> &(usize, usize) {
        &self.dimensions
    }

    pub fn scale(self, factor: usize) -> Self {
        let Self {
            palette,
            data,
            dimensions: (width, height),
            transparent,
        } = self;
        let dimensions = (width * factor, height * factor);
        let data = from_dimensions(&dimensions, |idx| {
            let index = scaled_palette_index(factor, idx, &dimensions);
            data[index]
        });
        Self {
            palette,
            data,
            dimensions,
            transparent,
        }
    }

    pub fn to_png(&self) -> Png {
        Png(image_data_to_png(self))
    }

    pub fn with_cursor(
        self,
        cursor_palette: &'static Palette,
        cursor_x: usize,
        cursor_y: usize,
    ) -> Layers<'c> {
        Layers::<'c>::new_background(self).set_cursor(cursor_palette, cursor_x, cursor_y)
    }
}
