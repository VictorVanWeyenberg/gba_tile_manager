mod image;
mod layer;
mod png;
mod render;

pub use image::ImageData;

// Render palette icon.
pub use render::render_palette;

// Render palette editing area.
// Use render::render_palette, pipe to new_background with cursor.
pub use layer::Layers;

// Render tilemap icon.
pub use render::render_tiles;

// Render tile editing area.
pub use render::render_tile; // pipe to new_background with cursor.
// pub use layer::Layers;

// Render screen icon.
// pub use layer::Layers;
// Use new_screen to png.

// Render screen editing area.
// pub use layer::Layers;
// Use new_screen with cursor to pngs.