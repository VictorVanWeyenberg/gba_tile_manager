use crate::error::Error;
use crate::project::Project;
use clap::Parser;
use std::path::PathBuf;
use std::{env, fs};

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
    /// Flattens the output artifacts into one directory.
    #[arg(short, long, default_value_t = false)]
    flatten: bool,
}

impl Args {
    pub fn input(&self) -> Result<PathBuf, Error> {
        if self.input.is_none() {
            env::current_dir().map_err(|e| Error::IO(e, "Current working directory".to_string()))
        } else {
            self.input.clone().ok_or_else(|| unreachable!())
        }
        .map(|input| {
            fs::canonicalize(&input)
                .unwrap_or_else(|_| panic!("Output path: {:?} does not exist", input))
        })
    }

    pub fn output(&self) -> Result<PathBuf, Error> {
        let output = if self.output.is_none() {
            env::current_dir().map_err(|e| Error::IO(e, "Current working directory".to_string()))
        } else {
            self.output.clone().ok_or_else(|| unreachable!())
        };

        if let Ok(output) = &output
            && !output.exists()
        {
            fs::create_dir_all(output)
                .map_err(|e| Error::IO(e, output.to_str().unwrap().to_string()))?;
        }

        output.map(|output| {
            fs::canonicalize(&output)
                .unwrap_or_else(|_| panic!("Output path: {:?} does not exist", output))
        })
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut project: Project = args.input()?.try_into()?;
    let paths = project.digest()?.save(args.output()?, args.flatten)?;
    for path in paths {
        println!("{}", path.to_str().unwrap());
    }
    Ok(())
}
