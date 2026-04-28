use crate::error::Error;
use crate::project::{BoopCsv, BoopRecord};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

const NORTH: (f32, f32) = (0.0, -1.0);
const EAST: (f32, f32) = (1.0, 0.0);
const SOUTH: (f32, f32) = (0.0, 1.0);
const WEST: (f32, f32) = (-1.0, 0.0);

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
    pub fn save<P: AsRef<Path>>(&self, dir: P) -> Result<PathBuf, Error> {
        let dir = dir.as_ref();

        let boops_path = dir.join(format!("{}_boops.bin", self.name));
        let mut boops_file = File::create(&boops_path)
            .map_err(|e| Error::IO(e, boops_path.to_str().unwrap().to_string()))?;
        let mut all_args = vec![];
        let boop_bytes = self
            .boops
            .iter()
            .flat_map(|boop| BoopBytes::new(boop, &mut all_args).as_bytes())
            .collect::<Vec<u8>>();
        boops_file
            .write_all(&boop_bytes)
            .map_err(|e| Error::IO(e, boops_path.to_str().unwrap().to_string()))?;

        let args_path = dir.join(format!("{}_boops_args.bin", self.name));
        let mut args_file = File::create(&args_path)
            .map_err(|e| Error::IO(e, args_path.to_str().unwrap().to_string()))?;
        args_file
            .write_all(&all_args)
            .map_err(|e| Error::IO(e, args_path.to_str().unwrap().to_string()))?;

        Ok(dir.to_path_buf())
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
            let north = best_boop_idx_for_direction(&from, &value, &NORTH);
            let east = best_boop_idx_for_direction(&from, &value, &EAST);
            let south = best_boop_idx_for_direction(&from, &value, &SOUTH);
            let west = best_boop_idx_for_direction(&from, &value, &WEST);
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
        all_args.extend_from_slice(&args);
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
