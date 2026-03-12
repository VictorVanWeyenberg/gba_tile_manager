use crate::palette::Palette;
use crate::project::VRamData;
use crate::render::image::BORDER_WIDTH;
use crate::render::png::Png;
use crate::render::render::render_background;
use crate::render::{ImageData, render_cursor, render_screen};

pub struct Layers<'c> {
    background: ImageData<'c>,
    layers: Vec<ImageData<'c>>,
    cursor: Option<ImageData<'c>>,
}

impl<'c> Layers<'c> {
    pub fn new(palette: &'c Palette, dimensions: (usize, usize)) -> Self {
        Self {
            background: render_background(palette, dimensions),
            layers: vec![],
            cursor: None,
        }
    }

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
            layers: vec![
                render_screen(palette, bg0_character_data, bg0_screen_data),
                render_screen(palette, bg1_character_data, bg1_screen_data),
            ],
            cursor: None,
        }
    }

    pub fn new_background(
        background: ImageData<'c>,
    ) -> Self {
        Self {
            background,
            layers: vec![],
            cursor: None,
        }
    }

    pub fn set_cursor(self, cursor_palette: &'c Palette, cursor_x: usize, cursor_y: usize) -> Self {
        let Self {
            background,
            layers,
            cursor,
        } = self;
        let (background, layers) = match cursor {
            None => (
                border(background),
                layers.into_iter().map(|layer| border(layer)).collect(),
            ),
            Some(_) => (background, layers),
        };

        let cursor = Some(render_cursor(
            cursor_palette,
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

    pub fn to_png(&self) -> Vec<Png> {
        let mut layers = vec![];
        layers.push(self.background.to_png());
        for layer in &self.layers {
            layers.push(layer.to_png());
        }
        match &self.cursor {
            Some(cursor) => layers.push(cursor.to_png()),
            None => {}
        }
        layers
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
