use crate::project::Project;
use std::env;
use std::path::PathBuf;
use clap::Parser;
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

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets the input directory. The place of the config file and the input artifacts.
    #[arg(short, long, value_name = "DIRECTORY")]
    input: Option<PathBuf>,
    /// Sets the output directory. The place for the binary artifacts to go.
    #[arg(short, long, value_name = "DIRECTORY")]
    output: Option<PathBuf>,
}

impl Args {
    pub fn input(&self) -> Result<PathBuf, Error> {
        if self.input.is_none() {
            env::current_dir().map_err(|e| Error::IO(e, "Current working directory".to_string()))
        } else {
            self.input.clone().ok_or_else(|| unreachable!())
        }
    }

    pub fn output(&self) -> Result<PathBuf, Error> {
        if self.output.is_none() {
            env::current_dir().map_err(|e| Error::IO(e, "Current working directory".to_string()))
        } else {
            self.output.clone().ok_or_else(|| unreachable!())
        }
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    println!("Input: {:?}", args.input()?.to_str());
    let mut project: Project = args.input()?.try_into()?;
    println!("Loaded project in {}.", project.name());
    println!("Output: {:?}", args.output()?.to_str());
    project.digest()?.save(args.output()?)
}
