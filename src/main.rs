use crate::project::Project;
use std::env;
use crate::error::Error;

mod boop;
mod character;
mod character_data;
mod color;
mod error;
mod palette;
mod project;
mod savable;
mod screen;
mod tile;
mod tile_iter;

fn main() -> Result<(), Error> {
    let path = env::current_dir()
        .map_err(|e| Error::IO(e, "Current working directory".to_string()))?;
    let mut project: Project = path.clone().try_into()?;
    println!("Loaded project in {}.", project.name());
    project.digest()?.save(path)
}
