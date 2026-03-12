use std::ops::Deref;
use crate::render::image::ImageData;

pub struct Png(pub Vec<u8>);

impl Deref for Png {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn image_data_to_png(image_data: &impl ImageData) -> Vec<u8> {
    let mut png = vec![];
    let palette: Vec<u8> = image_data.palette()
        .deref()
        .into_iter()
        .map(|color| color.as_png_rgb())
        .flatten()
        .collect();
    let data: Vec<u8> = image_data.data()
        .chunks_exact(2)
        .map(|idx| (idx[0] << 4) | idx[1])
        .collect();
    let trns = image_data.trns();

    let (width, height) = image_data.dimensions();
    let mut encoder = png::Encoder::new(&mut png, *width as u32, *height as u32);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Four);
    encoder.set_palette(palette);
    encoder.set_trns(trns);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
    writer.finish().unwrap();
    png
}
