mod domain;
mod png;
mod render;

use std::path::PathBuf;
pub use domain::*;
use crate::png::project_to_pngs;
use crate::project::Project;

fn main() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("resources");
    let project = Project::try_from(directory).unwrap();

    project_to_pngs(&project);
}
