use crate::palette::Palette;
use crate::project::VRamData;
use crate::render::image::BORDER_WIDTH;
use crate::render::png::Png;
use crate::render::render::{render_background, render_cursor, render_screen};
use crate::render::ImageData;

pub struct Layers<'c> {
    background: ImageData<'c>,
    layers: Option<(ImageData<'c>, ImageData<'c>)>,
    cursor: Option<ImageData<'c>>,
}

impl<'c> Layers<'c> {
    pub fn new_screen(
        palette: &'c Palette,
        VRamData {
            bg0_character_data,
            bg0_screen_data,
            bg1_character_data,
            bg1_screen_data,
        }: &VRamData,
    ) -> Self {
        Self {
            background: render_background(palette, (240, 160)),
            layers: Some((
                render_screen(palette, bg0_character_data, bg0_screen_data),
                render_screen(palette, bg1_character_data, bg1_screen_data),
            )),
            cursor: None,
        }
    }

    pub fn new_background(
        background: ImageData<'c>,
    ) -> Self {
        Self {
            background,
            layers: None,
            cursor: None,
        }
    }

    pub fn set_cursor(self, cursor_x: usize, cursor_y: usize) -> Self {
        let Self {
            background,
            layers,
            cursor,
        } = self;
        let (background, layers) = match cursor {
            None => (
                border(background),
                layers.map(|(bg0, bg1)| (border(bg0), border(bg1)))
            ),
            Some(_) => (background, layers),
        };

        let cursor = Some(render_cursor(
            background.dimensions,
            cursor_x,
            cursor_y,
        ));
        Self {
            background,
            layers,
            cursor,
        }
    }

    pub fn to_pngs(&self) -> Vec<Png> {
        let mut layers = vec![];
        layers.push(self.background.to_png());
        if let Some((bg0, bg1)) = &self.layers {
            layers.push(bg0.to_png());
            layers.push(bg1.to_png())
        }
        if let Some(cursor) = &self.cursor {
            layers.push(cursor.to_png())
        }
        layers
    }

    pub fn to_png(&self) -> Png {
        let palette = self.background.palette;
        let (width, height) = self.background.dimensions;
        let data = if let Some((bg0, bg1)) = &self.layers {
            let mut data = self.background.data.clone();
            bg1.data().iter()
                .zip(bg0.data.iter())
                .map(|(&b1, &b0)| if b0 != 0 { b0 } else { b1 })
                .enumerate()
                .for_each(|(idx, pal)| data[idx] = pal);
            data
        } else {
            self.background.data.clone()
        };
        ImageData {
            palette,
            data,
            dimensions: (width, height),
            transparent: false,
        }.to_png()
    }
}

fn border(image_data: ImageData) -> ImageData {
    let ImageData {
        palette,
        data,
        dimensions: (width, height),
        transparent,
    } = image_data;
    let new_width = width + 2 * BORDER_WIDTH;
    let new_height = height + 2 * BORDER_WIDTH;
    let mut new_data = vec![0u8; new_width * new_height];

    for y in 0..height {
        let src_row = y * width;
        let dst_row = (y + BORDER_WIDTH) * new_width;
        new_data[(dst_row + BORDER_WIDTH)..(dst_row + BORDER_WIDTH + width)]
            .copy_from_slice(&data[src_row..(src_row + width)]);
    }

    let new_dimensions = (new_width, new_height);
    ImageData {
        palette,
        data: new_data,
        dimensions: new_dimensions,
        transparent,
    }
}
