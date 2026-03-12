use crate::palette::Palette;
use crate::project::VRamData;
use crate::render::image::{ImageData, OpaqueImageData, TransparencyImageData};
use crate::render::render::render_background;
use crate::render::{render_cursor, render_screen};
use crate::render::png::Png;

pub struct Layers<'c> {
    background: OpaqueImageData<'c>,
    layers: Vec<TransparencyImageData<'c>>,
    cursor: Option<TransparencyImageData<'c>>,
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

    pub fn set_cursor(self, cursor_palette: &'c Palette, cursor_x: usize, cursor_y: usize) -> Self {
        let Self {
            background,
            layers,
            cursor,
        } = self;
        let (background, layers) = match cursor {
            None => (background.border(), layers.into_iter().map(|layer| layer.border()).collect()),
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
