use crate::error::Error;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub trait Savable: Sized {
    fn name(&self) -> &str;
    fn suffix() -> &'static str;
    fn create<R: Read>(name: impl ToString, data: R) -> Self;
    fn as_data(&self) -> Vec<u8>;
    fn read<P: AsRef<Path> + Debug>(path: P) -> Result<Self, Error> {
        let name = if let Some(file_name) = path.as_ref().file_name() {
            let file_name = file_name.to_str().unwrap();
            if file_name.ends_with(Self::suffix()) {
                file_name.replace(Self::suffix(), "")
            } else {
                return Err(format!(
                    "File is not a palette file (ending in `_palette.bin`) `{file_name}`"
                )
                .as_str()
                .into());
            }
        } else {
            return Err(format!("Unable to determine file name of path `{path:?}`")
                .as_str()
                .into());
        };

        let file = File::open(path)?;
        Ok(Self::create(name, file))
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, Error> {
        let file_name = format!("{}{}", self.name(), Self::suffix());
        let file_path = path.as_ref().join(file_name);
        let bytes: Vec<u8> = self.as_data();
        fs::write(&file_path, bytes)?;
        Ok(file_path)
    }
}