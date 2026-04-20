const IMAGE_WIDTH: usize = 256;
const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;

pub trait TiledIterExt: Iterator<Item = u8> + Sized {
    fn tiled(self) -> impl Iterator<Item = u8>;
    fn tile_chunked(self) -> Vec<[u8; 64]>;
}

impl<I: Iterator<Item = u8>> TiledIterExt for I {
    fn tiled(self) -> impl Iterator<Item = u8> {
        TiledIter::new(self)
    }

    fn tile_chunked(self) -> Vec<[u8; 64]> {
        self.collect::<Vec<_>>()
            .chunks(64)
            .map(|chunk| chunk.try_into().unwrap())
            .collect()
    }
}

struct TiledIter {
    data: Vec<u8>, // collected upfront since we need random access
    i: usize,
}

impl TiledIter {
    fn new(iter: impl Iterator<Item = u8>) -> Self {
        Self {
            data: iter.collect(),
            i: 0,
        }
    }
}

impl Iterator for TiledIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.i >= self.data.len() {
            return None;
        }

        let tiles_per_row = IMAGE_WIDTH / TILE_WIDTH;
        let tile_size = TILE_WIDTH * TILE_HEIGHT;

        let tile_idx = self.i / tile_size;
        let pixel_in_tile = self.i % tile_size;

        let tile_col = (tile_idx % tiles_per_row) * TILE_WIDTH;
        let tile_row = (tile_idx / tiles_per_row) * TILE_HEIGHT;

        let px = pixel_in_tile % TILE_WIDTH;
        let py = pixel_in_tile / TILE_WIDTH;

        let src_idx = (tile_row + py) * IMAGE_WIDTH + (tile_col + px);

        self.i += 1;
        self.data.get(src_idx).copied()
    }
}