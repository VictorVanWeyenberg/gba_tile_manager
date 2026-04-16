use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use crate::err::ProjectIOError;

// Flag bit positions
const FLAG_NORTH: u8 = 0;
const FLAG_EAST: u8 = 1;
const FLAG_SOUTH: u8 = 2;
const FLAG_WEST: u8 = 3;
const FLAG_CALLBACK: u8 = 4;
const FLAG_ARGS_INDEX: u8 = 5;

#[derive(Debug, Eq, PartialEq)]
pub struct Boop {
    sx: u8,
    sy: u8,
    ex: u8,
    ey: u8,
    flags: u8,
    north: Option<u8>,
    east: Option<u8>,
    south: Option<u8>,
    west: Option<u8>,
    callback_index: Option<u8>,
    args_index: Option<u8>,
    actual_args: Vec<u8>,
}

impl Boop {
    pub fn new(sx: u8, sy: u8, ex: u8, ey: u8) -> Self {
        Self {
            sx,
            sy,
            ex,
            ey,
            flags: 0,
            north: None,
            east: None,
            south: None,
            west: None,
            callback_index: None,
            args_index: None,
            actual_args: Vec::new(),
        }
    }

    fn set_flag(&mut self, bit: u8) {
        self.flags |= 1 << bit;
    }

    fn clear_flag(&mut self, bit: u8) {
        self.flags &= !(1 << bit);
    }

    pub fn set_north(&mut self, value: Option<u8>) {
        self.north = value;
        match value {
            Some(_) => self.set_flag(FLAG_NORTH),
            None => self.clear_flag(FLAG_NORTH),
        }
    }

    pub fn set_east(&mut self, value: Option<u8>) {
        self.east = value;
        match value {
            Some(_) => self.set_flag(FLAG_EAST),
            None => self.clear_flag(FLAG_EAST),
        }
    }

    pub fn set_south(&mut self, value: Option<u8>) {
        self.south = value;
        match value {
            Some(_) => self.set_flag(FLAG_SOUTH),
            None => self.clear_flag(FLAG_SOUTH),
        }
    }

    pub fn set_west(&mut self, value: Option<u8>) {
        self.west = value;
        match value {
            Some(_) => self.set_flag(FLAG_WEST),
            None => self.clear_flag(FLAG_WEST),
        }
    }

    pub fn set_callback_index(&mut self, value: Option<u8>) {
        self.callback_index = value;
        match value {
            Some(_) => self.set_flag(FLAG_CALLBACK),
            None => self.clear_flag(FLAG_CALLBACK),
        }
    }

    pub fn set_args(&mut self, args_index: Option<u8>, actual_args: Vec<u8>) {
        self.args_index = args_index;
        self.actual_args = actual_args;
        match args_index {
            Some(_) => self.set_flag(FLAG_ARGS_INDEX),
            None => self.clear_flag(FLAG_ARGS_INDEX),
        }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        [
            self.sx,
            self.sy,
            self.ex,
            self.ey,
            self.flags,
            self.north.unwrap_or(0),
            self.east.unwrap_or(0),
            self.south.unwrap_or(0),
            self.west.unwrap_or(0),
            self.callback_index.unwrap_or(0),
            self.args_index.unwrap_or(0),
            self.actual_args.len() as u8,
        ]
    }

    pub fn from_bytes(bytes: &[u8; 12], args_data: &[u8]) -> Self {
        let flags = bytes[4];
        let has_flag = |bit: u8| (flags >> bit) & 1 == 1;

        let north = if has_flag(FLAG_NORTH) { Some(bytes[5]) } else { None };
        let east = if has_flag(FLAG_EAST) { Some(bytes[6]) } else { None };
        let south = if has_flag(FLAG_SOUTH) { Some(bytes[7]) } else { None };
        let west = if has_flag(FLAG_WEST) { Some(bytes[8]) } else { None };
        let callback_index = if has_flag(FLAG_CALLBACK) { Some(bytes[9]) } else { None };
        let args_index = if has_flag(FLAG_ARGS_INDEX) { Some(bytes[10]) } else { None };
        let args_len = bytes[11] as usize;

        let actual_args = if let Some(idx) = args_index {
            let start = idx as usize;
            let end = start + args_len;
            if end <= args_data.len() {
                args_data[start..end].to_vec()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        Self {
            sx: bytes[0],
            sy: bytes[1],
            ex: bytes[2],
            ey: bytes[3],
            flags,
            north,
            east,
            south,
            west,
            callback_index,
            args_index,
            actual_args,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Boops {
    pub name: String,
    pub boops: Vec<Boop>,
}

impl Boops {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            boops: Vec::new(),
        }
    }

    /// Saves to `<dir>/<name>_boops.bin` and `<dir>/<name>_boops_args.bin`.
    /// Returns the directory path as a `PathBuf`.
    pub fn save<P: AsRef<Path>>(&self, dir: P) -> Result<PathBuf, ProjectIOError> {
        let dir = dir.as_ref();

        let boops_path = dir.join(format!("{}_boops.bin", self.name));
        let mut boops_file = File::create(&boops_path)?;
        for boop in &self.boops {
            boops_file.write_all(&boop.to_bytes())?;
        }

        let args_path = dir.join(format!("{}_boops_args.bin", self.name));
        let mut args_file = File::create(&args_path)?;
        for boop in &self.boops {
            if !boop.actual_args.is_empty() {
                args_file.write_all(&boop.actual_args)?;
            }
        }

        Ok(dir.to_path_buf())
    }

    /// Loads from the given `_boops.bin` path, deriving the name by stripping
    /// the `_boops.bin` suffix, and the args file as the sibling
    /// `_boops_args.bin` in the same directory.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ProjectIOError> {
        let boops_path = path.as_ref();

        let file_name = boops_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| ProjectIOError::Custom(
                "path has no valid file name".into(),
            ))?;

        let name = file_name
            .strip_suffix("_boops.bin")
            .ok_or_else(|| ProjectIOError::Custom(format!(
                "file name {file_name:?} does not end with `_boops.bin`",
            )))?
            .to_owned();

        let dir = boops_path.parent().unwrap_or(Path::new("."));
        let args_path = dir.join(format!("{name}_boops_args.bin"));

        let mut boops_data = Vec::new();
        File::open(boops_path)?.read_to_end(&mut boops_data)?;

        let mut args_data = Vec::new();
        if let Ok(mut f) = File::open(&args_path) {
            f.read_to_end(&mut args_data)?;
        }

        if boops_data.len() % 12 != 0 {
            return Err(ProjectIOError::Custom(format!(
                "boops file size {} is not a multiple of 12",
                boops_data.len()
            )));
        }

        let boops = boops_data
            .chunks_exact(12)
            .map(|chunk| {
                let bytes: &[u8; 12] = chunk.try_into().unwrap();
                Boop::from_bytes(bytes, &args_data)
            })
            .collect();

        Ok(Self { name, boops })
    }

    /// Assigns `args_index` values sequentially across all boops, then saves.
    pub fn assign_args_indices_and_save<P: AsRef<Path>>(
        &mut self,
        dir: P,
    ) -> Result<PathBuf, ProjectIOError> {
        let mut cursor: u8 = 0;
        for boop in &mut self.boops {
            if !boop.actual_args.is_empty() {
                let args = boop.actual_args.clone();
                let len = args.len() as u8;
                boop.set_args(Some(cursor), args);
                cursor = cursor
                    .checked_add(len)
                    .ok_or_else(|| ProjectIOError::Custom(
                        "args_index overflowed u8".into(),
                    ))?;
            }
        }
        self.save(dir)
    }

    /// Automatically fills in the north/east/south/west fields for every boop
    /// using a directional score of cos(θ) / distance, where θ is the angle
    /// between the cardinal direction vector and the vector to the candidate's
    /// center. Only candidates within a 90° cone (cos θ > 0) are considered.
    pub fn auto_fill_directions(&mut self) {
        // Cardinal direction unit vectors: (dx, dy) in screen space where
        // +y is down (typical for tilemaps / UI coordinates).
        const DIRECTIONS: [(f32, f32, usize); 4] = [
            ( 0.0, -1.0, 0), // north
            ( 1.0,  0.0, 1), // east
            ( 0.0,  1.0, 2), // south
            (-1.0,  0.0, 3), // west
        ];

        let centers: Vec<(f32, f32)> = self.boops.iter().map(|b| {
            (
                (b.sx as f32 + b.ex as f32) / 2.0,
                (b.sy as f32 + b.ey as f32) / 2.0,
            )
        }).collect();

        let len = self.boops.len();

        // For each boop, find the best candidate in each cardinal direction.
        // We collect results first to avoid borrowing self.boops mutably while
        // reading it.
        let mut results: Vec<[Option<u8>; 4]> = vec![[None; 4]; len];

        for i in 0..len {
            let (cx, cy) = centers[i];

            for &(dx, dy, dir_idx) in &DIRECTIONS {
                let mut best_score = 0.0f32; // cos(θ)/dist must beat this; cos(θ)>0 is the gate
                let mut best: Option<u8> = None;

                for j in 0..len {
                    if i == j {
                        continue;
                    }

                    let (tx, ty) = centers[j];
                    let vx = tx - cx;
                    let vy = ty - cy;
                    let dist = (vx * vx + vy * vy).sqrt();

                    if dist < f32::EPSILON {
                        continue; // coincident centers, skip
                    }

                    // cos(θ) = dot(dir, v̂)
                    let cos_theta = (dx * vx + dy * vy) / dist;

                    if cos_theta <= 0.0 {
                        continue; // outside the 90° forward cone
                    }

                    let score = cos_theta / dist;
                    if score > best_score {
                        best_score = score;
                        best = Some(j as u8);
                    }
                }

                results[i][dir_idx] = best;
            }
        }

        // Apply results.
        for (boop, dirs) in self.boops.iter_mut().zip(results.iter()) {
            boop.set_north(dirs[0]);
            boop.set_east(dirs[1]);
            boop.set_south(dirs[2]);
            boop.set_west(dirs[3]);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn roundtrip() {
        let tmp = TempDir::new("boops_test").unwrap();

        let mut boops = Boops::new("level01");

        let mut b0 = Boop::new(1, 2, 3, 4);
        b0.set_north(Some(10));
        b0.set_west(Some(20));
        b0.set_callback_index(Some(7));
        boops.push(b0);

        let mut b1 = Boop::new(5, 6, 7, 8);
        b1.set_east(Some(30));
        b1.set_south(Some(40));
        b1.actual_args = vec![0xAA, 0xBB, 0xCC];
        boops.push(b1);

        let mut b2 = Boop::new(9, 10, 11, 12);
        b2.set_north(Some(50));
        b2.set_east(Some(60));
        b2.set_south(Some(70));
        b2.set_west(Some(80));
        b2.actual_args = vec![0x01, 0x02];
        boops.push(b2);

        boops.assign_args_indices_and_save(tmp.path()).unwrap();

        let expected_bytes: Vec<[u8; 12]> = boops.iter().map(|b| b.to_bytes()).collect();
        let expected_args: Vec<Vec<u8>> = boops.iter().map(|b| b.actual_args.clone()).collect();

        // Load by passing the _boops.bin path directly.
        let boops_bin = tmp.path().join("level01_boops.bin");
        let loaded = Boops::load(&boops_bin).unwrap();

        assert_eq!(loaded.name, "level01");
        assert_eq!(loaded.len(), 3);

        for (i, (boop, exp_bytes)) in loaded.iter().zip(expected_bytes.iter()).enumerate() {
            assert_eq!(boop.to_bytes(), *exp_bytes, "boop[{i}] bytes mismatch");
            assert_eq!(boop.actual_args, expected_args[i], "boop[{i}] actual_args mismatch");
        }
    }

    /// Layout (centers marked with *):
    ///
    ///          [N]
    ///           *  (10,0)-(20,10)
    ///
    ///  [W]*          *[E]
    /// (0,10)-(10,20)   (20,10)-(30,20)
    ///
    ///          [C]
    ///           *  (10,10)-(20,20)  <- center boop
    ///
    ///          [S]
    ///           *  (10,20)-(20,30)
    #[test]
    fn auto_fill_directions_cross() {
        let mut boops = Boops::new("test");

        // index 0: center
        boops.push(Boop::new(10, 10, 20, 20));
        // index 1: north
        boops.push(Boop::new(10, 0, 20, 10));
        // index 2: east
        boops.push(Boop::new(20, 10, 30, 20));
        // index 3: south
        boops.push(Boop::new(10, 20, 20, 30));
        // index 4: west
        boops.push(Boop::new(0, 10, 10, 20));

        boops.auto_fill_directions();


        let c = &boops.boops[0];
        assert_eq!(c.north, Some(1), "center -> north");
        assert_eq!(c.east,  Some(2), "center -> east");
        assert_eq!(c.south, Some(3), "center -> south");
        assert_eq!(c.west,  Some(4), "center -> west");

        let n = &boops.boops[1];
        assert_eq!(n.north, None,    "north boop: nothing further north");
        assert_eq!(n.east,  Some(2), "north boop: east neighbour");
        assert_eq!(n.south, Some(0), "north boop: south back to center");
        assert_eq!(n.west,  Some(4), "north boop: west neighbour");

        let e = &boops.boops[2];
        assert_eq!(e.north, Some(1), "east boop: north neighbour");
        assert_eq!(e.east,  None,    "east boop: nothing further east");
        assert_eq!(e.south, Some(3), "east boop: south neighbour");
        assert_eq!(e.west,  Some(0), "east boop: west back to center");

        let s = &boops.boops[3];
        assert_eq!(s.north, Some(0), "south boop: north back to center");
        assert_eq!(s.east,  Some(2), "south boop: east neighbour");
        assert_eq!(s.south, None,    "south boop: nothing further south");
        assert_eq!(s.west,  Some(4), "south boop: west neighbour");

        let w = &boops.boops[4];
        assert_eq!(w.north, Some(1), "west boop: north neighbour");
        assert_eq!(w.east,  Some(0), "west boop: east back to center");
        assert_eq!(w.south, Some(3), "west boop: south neighbour");
        assert_eq!(w.west,  None,    "west boop: nothing further west");
    }
}