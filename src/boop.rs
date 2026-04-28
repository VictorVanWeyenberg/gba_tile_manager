use crate::error::Error;
use crate::project::{BoopCsv, BoopRecord};
use png::{BitDepth, ColorType, Encoder};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

const NORTH: (f32, f32) = (0.0, -1.0);
const EAST: (f32, f32) = (1.0, 0.0);
const SOUTH: (f32, f32) = (0.0, 1.0);
const WEST: (f32, f32) = (-1.0, 0.0);
// Palette indices
const IDX_TRANSPARENT: u8 = 0; // black, fully transparent
const IDX_BORDER_FILL: u8 = 1; // grey, half-transparent (border body)
const IDX_BORDER_EDGE: u8 = 2; // grey, half-transparent (same, kept for clarity)
const IDX_NORTH: u8 = 3;       // red,    full opacity
const IDX_EAST: u8 = 4;        // yellow, full opacity
const IDX_SOUTH: u8 = 5;       // green,  full opacity
const IDX_WEST: u8 = 6;        // blue,   full opacity

const W: usize = 256;
const LINK_OFFSET: i32 = 4; // pixel offset so opposing links don't overlap

#[derive(Debug, Eq, PartialEq)]
pub struct Boop {
    x: u8,
    y: u8,
    w: u8,
    h: u8,
    north: Option<u8>,
    east: Option<u8>,
    south: Option<u8>,
    west: Option<u8>,
    callback: Option<u8>,
    args: Vec<u8>,
}

impl Boop {
    pub fn new(
        record: &BoopRecord,
        north: Option<u8>,
        east: Option<u8>,
        south: Option<u8>,
        west: Option<u8>,
    ) -> Self {
        let BoopRecord {
            x,
            y,
            w,
            h,
            callback,
            args,
        } = record;
        Self {
            x: *x,
            y: *y,
            w: *w,
            h: *h,
            north,
            east,
            south,
            west,
            callback: *callback,
            args: args.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Boops {
    pub name: String,
    pub boops: Vec<Boop>,
}

impl Boops {
    pub fn new(name: impl ToString, boops: Vec<Boop>) -> Self {
        Self {
            name: name.to_string(),
            boops,
        }
    }

    /// Saves to `<dir>/<name>_boops.bin` and `<dir>/<name>_boops_args.bin`.
    /// Returns the directory path as a `PathBuf`.
    pub fn save<P: AsRef<Path>>(&self, dir: P, flatten: bool) -> Result<PathBuf, Error> {
        let dir = dir.as_ref();
        let mut all_args = vec![];
        let boop_bytes = self
            .boops
            .iter()
            .flat_map(|boop| BoopBytes::new(boop, &mut all_args).as_bytes())
            .collect::<Vec<u8>>();

        self.write_boops_file(dir, flatten, "_boops.bin", boop_bytes)?;
        self.write_boops_file(dir, flatten, "_boops_args.bin", all_args)?;
        self.write_to_png(dir, flatten)?;

        Ok(dir.to_path_buf())
    }

    fn write_boops_file(&self, dir: &Path, flatten: bool, suffix: &str, boop_bytes: Vec<u8>) -> Result<(), Error> {
        let boops_file_name = self.flatten_and_suffix(flatten, suffix);
        let boops_path = dir.join(boops_file_name.clone());
        let mut boops_file = File::create(&boops_path)
            .map_err(|e| Error::IO(e, boops_path.to_str().unwrap().to_string()))?;
        boops_file
            .write_all(&boop_bytes)
            .map_err(|e| Error::IO(e, boops_path.to_str().unwrap().to_string()))?;
        Ok(())
    }

    fn flatten_and_suffix(&self, flatten: bool, suffix: &str) -> String {
        format!("{}{}", if flatten {
            self.name.replace("/", "_")
        } else {
            self.name.clone()
        }, suffix)
    }

    fn write_to_png(&self, dir: &Path, flatten: bool) -> Result<(), Error> {
        let boops_file_name = self.flatten_and_suffix(flatten, "_boops.png");
        let boops_path = dir.join(boops_file_name.clone());
        let file = File::create(boops_path).unwrap();
        let writer = BufWriter::new(file);
        let data = self.to_png()?;

        // --- encode PNG ---
        let mut encoder = Encoder::new(writer, W as u32, W as u32);
        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(BitDepth::Eight);

        // PLTE: RGB triples
        encoder.set_palette(vec![
            0,   0,   0,   // 0: transparent black
            128, 128, 128, // 1: grey (border)
            128, 128, 128, // 2: grey (border edge, same)
            255, 0,   0,   // 3: red   (north)
            255, 255, 0,   // 4: yellow (east)
            0,   255, 0,   // 5: green  (south)
            0,   0,   255, // 6: blue   (west)
        ]);

        // tRNS: alpha per palette entry
        encoder.set_trns(vec![
            0,   // 0: fully transparent
            128, // 1: half transparent
            128, // 2: half transparent
            255, // 3: full opacity
            255, // 4: full opacity
            255, // 5: full opacity
            255, // 6: full opacity
        ]);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&data)?;
        Ok(writer.finish()?)
    }

    fn to_png(&self) -> Result<Vec<u8>, Error> {
        let mut canvas = vec![IDX_TRANSPARENT; W * W];

        // --- draw boop borders (1-pixel border only) ---
        for boop in &self.boops {
            let (x, y, w, h) = (
                boop.x as usize,
                boop.y as usize,
                boop.w as usize,
                boop.h as usize,
            );
            // top and bottom edges
            for dx in 0..w {
                canvas[y * W + x + dx] = IDX_BORDER_FILL;
                canvas[(y + h - 1) * W + x + dx] = IDX_BORDER_FILL;
            }
            // left and right edges
            for dy in 0..h {
                canvas[(y + dy) * W + x] = IDX_BORDER_FILL;
                canvas[(y + dy) * W + x + w - 1] = IDX_BORDER_FILL;
            }
        }

        // --- draw directional links ---
        let draw_line = |canvas: &mut Vec<u8>, x0: i32, y0: i32, x1: i32, y1: i32, idx: u8| {
            let dx = (x1 - x0).abs();
            let dy = (y1 - y0).abs();
            let sx = if x0 < x1 { 1i32 } else { -1 };
            let sy = if y0 < y1 { 1i32 } else { -1 };
            let mut err = dx - dy;
            let (mut cx, mut cy) = (x0, y0);
            loop {
                if cx >= 0 && cy >= 0 && (cx as usize) < W && (cy as usize) < W {
                    canvas[cy as usize * W + cx as usize] = idx;
                }
                if cx == x1 && cy == y1 { break; }
                let e2 = 2 * err;
                if e2 > -dy { err -= dy; cx += sx; }
                if e2 < dx  { err += dx; cy += sy; }
            }
        };

        for boop in &self.boops {
            let cx = boop.x as i32 + boop.w as i32 / 2;
            let cy = boop.y as i32 + boop.h as i32 / 2;
            let top    = boop.y as i32;
            let bottom = boop.y as i32 + boop.h as i32 - 1;
            let left   = boop.x as i32;
            let right  = boop.x as i32 + boop.w as i32 - 1;

            // North link: leaves from top-center (offset left), arrives at bottom-center of dest (offset left)
            if let Some(ni) = boop.north {
                if let Some(dest) = self.boops.get(ni as usize) {
                    let dest_cx = dest.x as i32 + dest.w as i32 / 2;
                    let dest_bottom = dest.y as i32 + dest.h as i32 - 1;
                    draw_line(
                        &mut canvas,
                        cx - LINK_OFFSET, top,
                        dest_cx - LINK_OFFSET, dest_bottom,
                        IDX_NORTH,
                    );
                }
            }

            // East link: leaves from right-center (offset up), arrives at left-center of dest (offset up)
            if let Some(ei) = boop.east {
                if let Some(dest) = self.boops.get(ei as usize) {
                    let dest_left  = dest.x as i32;
                    let dest_cy    = dest.y as i32 + dest.h as i32 / 2;
                    draw_line(
                        &mut canvas,
                        right, cy - LINK_OFFSET,
                        dest_left, dest_cy - LINK_OFFSET,
                        IDX_EAST,
                    );
                }
            }

            // South link: leaves from bottom-center (offset right), arrives at top-center of dest (offset right)
            if let Some(si) = boop.south {
                if let Some(dest) = self.boops.get(si as usize) {
                    let dest_cx  = dest.x as i32 + dest.w as i32 / 2;
                    let dest_top = dest.y as i32;
                    draw_line(
                        &mut canvas,
                        cx + LINK_OFFSET, bottom,
                        dest_cx + LINK_OFFSET, dest_top,
                        IDX_SOUTH,
                    );
                }
            }

            // West link: leaves from left-center (offset down), arrives at right-center of dest (offset down)
            if let Some(wi) = boop.west {
                if let Some(dest) = self.boops.get(wi as usize) {
                    let dest_right = dest.x as i32 + dest.w as i32 - 1;
                    let dest_cy    = dest.y as i32 + dest.h as i32 / 2;
                    draw_line(
                        &mut canvas,
                        left, cy + LINK_OFFSET,
                        dest_right, dest_cy + LINK_OFFSET,
                        IDX_WEST,
                    );
                }
            }
        }

        Ok(canvas)
    }
}

fn score_boop_for_direction(
    from: &BoopRecord,
    to: &BoopRecord,
    (dx, dy): &(f32, f32),
) -> Option<f32> {
    let (cx, cy) = from.center();
    let (tx, ty) = to.center();
    let vx = tx - cx;
    let vy = ty - cy;
    let dist = (vx * vx + vy * vy).sqrt();

    if dist < f32::EPSILON {
        return None; // coincident centers, skip
    }

    // cos(θ) = dot(dir, v̂)
    let cos_theta = (dx * vx + dy * vy) / dist;

    if cos_theta <= 0.0 {
        return None; // outside the 90° forward cone
    }

    Some(cos_theta / dist)
}

fn best_boop_idx_for_direction(
    from: &BoopRecord,
    records: &BoopCsv,
    direction: &(f32, f32),
) -> Option<u8> {
    records
        .iter()
        .enumerate()
        .filter_map(|(idx, to)| {
            score_boop_for_direction(from, to, direction).map(|score| (idx, score))
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Less))
        .map(|(idx, _)| idx as u8)
}

impl From<BoopCsv> for Boops {
    fn from(value: BoopCsv) -> Self {
        let mut boops = vec![];
        for from in value.iter() {
            let north = best_boop_idx_for_direction(from, &value, &NORTH);
            let east = best_boop_idx_for_direction(from, &value, &EAST);
            let south = best_boop_idx_for_direction(from, &value, &SOUTH);
            let west = best_boop_idx_for_direction(from, &value, &WEST);
            boops.push(Boop::new(from, north, east, south, west))
        }
        Boops::new(value.name(), boops)
    }
}

impl Deref for Boops {
    type Target = Vec<Boop>;

    fn deref(&self) -> &Self::Target {
        &self.boops
    }
}

impl DerefMut for Boops {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.boops
    }
}

struct BoopBytes {
    x: u8,
    y: u8,
    w: u8,
    h: u8,
    north: u8,
    east: u8,
    south: u8,
    west: u8,
    callback: u8,
    args_index: u8,
    args_len: u8,
}

impl BoopBytes {
    pub fn new(boop: &Boop, all_args: &mut Vec<u8>) -> Self {
        let Boop {
            x,
            y,
            w,
            h,
            north,
            east,
            south,
            west,
            callback,
            args,
        } = boop;
        let boop_bytes = Self {
            x: *x,
            y: *y,
            w: *w,
            h: *h,
            north: north.unwrap_or(255),
            east: east.unwrap_or(255),
            south: south.unwrap_or(255),
            west: west.unwrap_or(255),
            callback: callback.unwrap_or(255),
            args_index: if args.is_empty() {
                0
            } else {
                all_args.len() as u8
            },
            args_len: args.len() as u8,
        };
        all_args.extend_from_slice(args);
        boop_bytes
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        vec![
            self.x,
            self.y,
            self.w,
            self.h,
            self.north,
            self.east,
            self.south,
            self.west,
            self.callback,
            self.args_index,
            self.args_len,
        ]
    }
}
