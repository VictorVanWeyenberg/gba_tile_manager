use crate::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Savable: Sized {
    fn name(&self) -> &str;
    fn as_data(&self) -> Vec<u8>;
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, Error> {
        let file_name = format!("{}.bin", self.name());
        let file_path = path.as_ref().join(file_name);
        let parent = file_path.parent().unwrap();
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| Error::IO(e, parent.to_str().unwrap().to_string()))?
        }
        let bytes: Vec<u8> = self.as_data();
        fs::write(&file_path, bytes)
            .map_err(|e| Error::IO(e, file_path.as_path().to_str().unwrap().to_string()))?;
        Ok(file_path)
    }
}
