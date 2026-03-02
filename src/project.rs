use crate::map::TileMap;
use crate::palette::Palette;
use crate::screen::Screen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Structure {
    name: String,
    screens: Vec<String>,
}

#[derive(Debug)]
struct VRamData {
    bg0_character_data: TileMap,
    bg0_screen_data: Screen,
    bg1_character_data: TileMap,
    bg1_screen_data: Screen,
}

#[derive(Debug)]
pub struct Project {
    name: String,
    background_palette: Palette,
    object_palette: Palette,
    object_character_data: TileMap,
    screens: HashMap<String, VRamData>,
}

#[derive(Debug)]
pub enum OpenProjectError {
    IO(std::io::Error),
    Serde(serde_json::Error),
}

impl Error for OpenProjectError {

}

impl Display for OpenProjectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for OpenProjectError {
    fn from(value: std::io::Error) -> Self {
        OpenProjectError::IO(value)
    }
}

impl From<serde_json::Error> for OpenProjectError {
    fn from(value: serde_json::Error) -> Self {
        OpenProjectError::Serde(value)
    }
}

impl TryFrom<PathBuf> for Project
{
    type Error = OpenProjectError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let Structure { name, screens } = read_structure(&path)?;
        let background_palette = read_palette(&path, "background_palette.bin")?;
        let object_palette = read_palette(&path, "object_palette.bin")?;
        let object_character_data = read_character_data(&path, "object_character_data.bin")?;
        let screens = screens.into_iter()
            .map(|name| read_vram_data(&path, name))
            .collect::<Result<HashMap<String, VRamData>, OpenProjectError>>()?;
        Ok(Project {
            name,
            background_palette,
            object_palette,
            object_character_data,
            screens,
        })
    }
}

fn read_structure(path: &PathBuf) -> Result<Structure, OpenProjectError> {
    let structure_location = path.join("structure.json");
    let file = File::open(structure_location)?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

fn read_palette(path: &PathBuf, file_name: &str) -> Result<Palette, OpenProjectError> {
    let palette_location = path.join(file_name);
    let file = File::open(palette_location)?;
    Ok(Palette::from(file))
}

fn read_character_data(path: &PathBuf, file_name: &str) -> Result<TileMap, OpenProjectError> {
    let tile_map_location = path.join(file_name);
    let file = File::open(tile_map_location)?;
    Ok(TileMap::from(file))
}

fn read_vram_data(path: &PathBuf, screen_name: String) -> Result<(String, VRamData), OpenProjectError> {
    let bg0_character_data_file_name = format!("bg0_{}_character_data.bin", screen_name);
    let bg1_character_data_file_name = format!("bg1_{}_character_data.bin", screen_name);
    let bg0_screen_data_file_name = format!("bg0_{}_screen_data.bin", screen_name);
    let bg1_screen_data_file_name = format!("bg1_{}_screen_data.bin", screen_name);

    let bg0_character_data = read_character_data(path, &bg0_character_data_file_name)?;
    let bg1_character_data = read_character_data(path, &bg1_character_data_file_name)?;
    let bg0_screen_data = read_screen_data(path, &bg0_screen_data_file_name)?;
    let bg1_screen_data = read_screen_data(path, &bg1_screen_data_file_name)?;

    Ok((screen_name, VRamData {
        bg0_character_data,
        bg1_character_data,
        bg0_screen_data,
        bg1_screen_data,
    }))
}

fn read_screen_data(path: &PathBuf, file_name: &str) -> Result<Screen, OpenProjectError> {
    let screen_location = path.join(file_name);
    let bytes = fs::read(screen_location)?;
    Ok(Screen::from(bytes))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::project::Project;

    #[test]
    fn read_project() {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("resources");
        let project = Project::try_from(directory).unwrap();
        println!("{:?}", project)
    }
}