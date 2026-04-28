use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use regex::Regex;
use crate::error::Error;

pub struct BoopCsv {
    name: String,
    records: Vec<BoopRecord>,
}

impl Deref for BoopCsv {
    type Target = Vec<BoopRecord>;
    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

impl DerefMut for BoopCsv {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.records
    }
}

impl BoopCsv {
    pub fn new(name: impl ToString, records: Vec<BoopRecord>) -> Self {
        BoopCsv { name: name.to_string(), records }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct BoopRecord {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
    pub callback: Option<u8>,
    pub args: Vec<u8>,
}

impl TryFrom<String> for BoopRecord {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.replace(" ", "");
        let regex = Regex::new(r"^(?<x>\d+),(?<y>\d+),(?<w>\d+),(?<h>\d+),(?<callback>\d*),(?<args>(\d+\|?)*)$")?;
        regex.captures(&value)
            .ok_or("No capture group found".to_string())
            .map_err(Error::Custom)
            .and_then(|captures| {
                let x = u8::from_str(captures.name("x").ok_or("No x.")?.as_str())?;
                let y = u8::from_str(captures.name("y").ok_or("No y.")?.as_str())?;
                let w = u8::from_str(captures.name("w").ok_or("No w.")?.as_str())?;
                let h = u8::from_str(captures.name("h").ok_or("No h.")?.as_str())?;
                let callback = captures.name("callback").ok_or("No callback.")?.as_str();
                let callback = if callback.is_empty() {
                    None
                } else {
                    Some(callback.parse::<u8>()?)
                };
                let args = captures.name("args").ok_or("No args.")?.as_str();
                let args = if args.is_empty() {
                    vec![]
                } else {
                    args.split("|")
                        .map(u8::from_str)
                        .collect::<Result<Vec<_>, _>>()?
                };
                Ok(BoopRecord { x, y, w, h, callback, args })
            })
    }
}

impl BoopRecord {
    pub fn center(&self) -> (f32, f32) {
        (
            self.x as f32 + self.w as f32 / 2.0,
            self.y as f32 + self.h as f32 / 2.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::project::csv::BoopRecord;

    #[test]
    fn try_from_string() {
        let s = "1,2,3,4,5,6".to_string();
        let _: BoopRecord = s.try_into().unwrap();
        let s = "1,2,3,4,,6|7".to_string();
        let _: BoopRecord = s.try_into().unwrap();
        let s = "1,2,3,4,,".to_string();
        let _: BoopRecord = s.try_into().unwrap();
    }
}