use std::borrow::Cow;
use crate::color::Color;
use crate::render::png::{image_data_to_png, Png};
use crate::render::render::{from_dimensions, scaled_palette_index};

pub trait ImageData {
    fn palette(&self) -> &Vec<&Color>;
    fn data(&self) -> &Vec<u8>;
    fn trns(&self) -> impl Into<Cow<'_, [u8]>>;
    fn dimensions(&self) -> &(usize, usize);
    fn scale(self, factor: usize) -> Self;
    fn to_png(self) -> Png;
}

/// For a 3x2 image, image data will have data [1, 2, 3, 4, 5, 6] that's supposed to be rendered as
/// follows.
///
/// ```
/// 1 2 3
/// 4 5 6
/// ```
pub struct OpaqueImageData<'c, const N: usize> {
    pub palette: Vec<&'c Color>,
    pub data: Vec<u8>,
    pub dimensions: (usize, usize),
}

impl<'c, const N: usize> ImageData for OpaqueImageData<'c, N> {
    fn palette(&self) -> &Vec<&Color> {
        &self.palette
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }

    fn trns(&self) -> impl Into<Cow<'_, [u8]>> {
        &[255; N]
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
}

pub struct TransparencyImageData<'c, const N: usize> {
    pub opaque: OpaqueImageData<'c, N>,
    pub trns: Vec<u8>,
}

impl<'c, const N: usize> ImageData for TransparencyImageData<'c, N> {
    fn palette(&self) -> &Vec<&Color> {
        &self.opaque.palette
    }

    fn data(&self) -> &Vec<u8> {
        &self.opaque.data
    }

    fn trns(&self) -> impl Into<Cow<'_, [u8]>> {
        &self.trns
    }

    fn dimensions(&self) -> &(usize, usize) {
        &self.opaque.dimensions
    }

    fn scale(self, factor: usize) -> Self {
        let Self { opaque, trns } = self;
        let opaque = opaque.scale(factor);
        let trns = from_dimensions(opaque.dimensions(), |idx| {
            let index = scaled_palette_index(factor, idx, opaque.dimensions());
            trns[index]
        });
        Self { opaque, trns }
    }

    fn to_png(self) -> Png {
        Png(image_data_to_png(self))
    }
}