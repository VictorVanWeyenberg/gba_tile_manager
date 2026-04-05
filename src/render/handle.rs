use iced::widget::image::Handle;
use crate::color::Color;
use crate::palette::Palette;
use crate::render::ImageData;

pub fn image_data_to_handle(ImageData { palette, data, dimensions: (width, height), transparent } : &ImageData) -> Handle {
    let data: Vec<u8> = data.iter()
        .map(|pal_idx| get_color(palette, pal_idx, transparent))
        .flatten()
        .collect();
    Handle::from_rgba(*width as u32, *height as u32, data)
}

fn get_color(palette: &Palette, pal_idx: &u8, transparent: &bool) -> [u8; 4] {
    match palette {
        Palette::Cursor => {
            if *pal_idx == 0 {
                [0, 0, 0, 0]
            } else {
                [128, 128, 128, 128]
            }
        }
        Palette::Gba { colors, .. } => {
            colors.get(*pal_idx as usize).cloned().unwrap_or(Color::black()).as_rgba(*transparent)
        }
    }
}