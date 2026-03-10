use crate::color::Color;
use crate::render::png::{image_data_to_png, Png};
use crate::render::render::{from_dimensions, scaled_palette_index};

pub trait ImageData {
    fn palette(&self) -> &Vec<&Color>;
    fn data(&self) -> &Vec<u8>;
    fn trns(&self) -> Vec<u8>;
    fn dimensions(&self) -> &(usize, usize);
    fn scale(self, factor: usize) -> Self;
    fn to_png(self) -> Png;
    fn border(self, size: usize) -> Self;
}

/// For a 3x2 image, image data will have data [1, 2, 3, 4, 5, 6] that's supposed to be rendered as
/// follows.
///
/// ```
/// 1 2 3
/// 4 5 6
/// ```
pub struct OpaqueImageData<'c> {
    pub palette: Vec<&'c Color>,
    pub data: Vec<u8>,
    pub dimensions: (usize, usize),
}

impl<'c> ImageData for OpaqueImageData<'c> {
    fn palette(&self) -> &Vec<&Color> {
        &self.palette
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }

    fn trns(&self) -> Vec<u8> {
        let (w, h) = &self.dimensions;
        vec![255; w * h]
    }

    fn dimensions(&self) -> &(usize, usize) {
        &self.dimensions
    }

    fn scale(self, factor: usize) -> Self {
        let Self {
            palette,
            data,
            dimensions: (width, height),
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
        }
    }

    fn to_png(self) -> Png {
        Png(image_data_to_png(self))
    }

    fn border(self, size: usize) -> Self {
        let Self {
            palette,
            data,
            dimensions,
        } = self;
        let (data, dimensions) = border_buffer(data, dimensions, size);
        Self {
            palette,
            dimensions,
            data,
        }
    }
}

pub struct TransparencyImageData<'c> {
    pub opaque: OpaqueImageData<'c>,
}

impl<'c> ImageData for TransparencyImageData<'c> {
    fn palette(&self) -> &Vec<&Color> {
        &self.opaque.palette
    }

    fn data(&self) -> &Vec<u8> {
        &self.opaque.data
    }

    fn trns(&self) -> Vec<u8> {
        let mut trns = vec![255; self.opaque.palette.len()];
        trns[0] = 0;
        trns
    }

    fn dimensions(&self) -> &(usize, usize) {
        &self.opaque.dimensions
    }

    fn scale(self, factor: usize) -> Self {
        let Self { opaque } = self;
        let opaque = opaque.scale(factor);
        Self { opaque }
    }

    fn to_png(self) -> Png {
        Png(image_data_to_png(self))
    }

    fn border(self, size: usize) -> Self {
        Self {
            opaque: self.opaque.border(size),
        }
    }
}

fn border_buffer(
    data: Vec<u8>,
    (width, height): (usize, usize),
    size: usize,
) -> (Vec<u8>, (usize, usize)) {
    let new_width = width + 2 * size;
    let new_height = height + 2 * size;
    let mut new_data = vec![0u8; new_width * new_height];

    for y in 0..height {
        let src_row = y * width;
        let dst_row = (y + size) * new_width;
        new_data[(dst_row + size)..(dst_row + size + width)]
            .copy_from_slice(&data[src_row..(src_row + width)]);
    }

    let new_dimensions = (new_width, new_height);
    (new_data, new_dimensions)
}
