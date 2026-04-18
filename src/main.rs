use crate::project::Project;
use std::env;

mod boop;
mod character;
mod color;
mod error;
mod map;
mod palette;
mod project;
mod savable;
mod screen;
mod tile;

fn main() {
    let project: Project = env::current_dir()
        .expect("Could not get current working directory.")
        .try_into()
        .expect("Failed to load project.");
    println!("Loaded project in {}.", project.name());
}
