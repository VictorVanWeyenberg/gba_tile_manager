use crate::palette::Palette;
use crate::render::handle::image_data_to_handle;
use iced::widget::image::Handle;

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
    pub(crate) fn to_handle(&self) -> Handle {
        image_data_to_handle(self)
    }
}
