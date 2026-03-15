use iced::widget::image::Handle;
use crate::render::ImageData;

pub fn image_data_to_handle(ImageData { palette, data, dimensions: (width, height), transparent } : &ImageData) -> Handle {
    let data: Vec<u8> = data.iter()
        .map(|pal_idx| &palette[*pal_idx as usize])
        .map(|color| color.as_rgba(*transparent))
        .flatten()
        .collect();
    Handle::from_rgba(*width as u32, *height as u32, data)
}