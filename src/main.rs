use crate::project::Project;
use std::env;

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

fn main() {
    let mut project: Project = env::current_dir()
        .expect("Could not get current working directory.")
        .try_into()
        .expect("Failed to load project.");
    println!("Loaded project in {}.", project.name());
    project.digest().expect("Could not digest project.");
}
