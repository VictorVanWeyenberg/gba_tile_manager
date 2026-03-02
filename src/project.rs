use crate::err::ProjectIOError;
use crate::map::TileMap;
use crate::palette::Palette;
use crate::screen::Screen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    path: PathBuf,
    background_palette: Palette,
    object_palette: Palette,
    object_character_data: TileMap,
    screens: HashMap<String, VRamData>,
}

impl Project {
    pub fn save(&self) -> Result<(), ProjectIOError> {
        let Project {
            name,
            path,
            background_palette,
            object_palette,
            object_character_data,
            screens,
        } = self;
        // TODO: Write to temp dir, then move.
        write_structure(
            path,
            Structure {
                name: name.to_string(),
                screens: screens.keys().cloned().collect(),
            },
        )?;
        write_palette(path, "background_palette.bin", background_palette)?;
        write_palette(path, "object_palette.bin", object_palette)?;
        write_character_data(path, "object_character_data.bin", object_character_data)?;
        for (name, vram_data) in screens {
            write_vram_data(path, name, vram_data)?;
        }
        Ok(())
    }
}

fn write_structure(path: &PathBuf, structure: Structure) -> Result<(), ProjectIOError> {
    let structure_location = path.join("structure.json");
    Ok(fs::write(
        structure_location,
        serde_json::to_string(&structure)?,
    )?)
}

fn write_palette(path: &PathBuf, file_name: &str, palette: &Palette) -> Result<(), ProjectIOError> {
    let palette_location = path.join(file_name);
    let bytes: Vec<u8> = palette.into();
    Ok(fs::write(palette_location, bytes)?)
}

fn write_character_data(
    path: &PathBuf,
    file_name: &str,
    tile_map: &TileMap,
) -> Result<(), ProjectIOError> {
    let character_data_location = path.join(file_name);
    let bytes: Vec<u8> = tile_map.into();
    Ok(fs::write(character_data_location, bytes)?)
}

fn write_vram_data(
    path: &PathBuf,
    screen_name: &str,
    VRamData {
        bg0_character_data,
        bg0_screen_data,
        bg1_character_data,
        bg1_screen_data,
    }: &VRamData,
) -> Result<(), ProjectIOError> {
    let bg0_character_data_file_name = format!("bg0_{}_character_data.bin", screen_name);
    let bg1_character_data_file_name = format!("bg1_{}_character_data.bin", screen_name);
    let bg0_screen_data_file_name = format!("bg0_{}_screen_data.bin", screen_name);
    let bg1_screen_data_file_name = format!("bg1_{}_screen_data.bin", screen_name);

    write_character_data(path, &bg0_character_data_file_name, bg0_character_data)?;
    write_screen_data(path, &bg0_screen_data_file_name, bg0_screen_data)?;
    write_character_data(path, &bg1_character_data_file_name, bg1_character_data)?;
    write_screen_data(path, &bg1_screen_data_file_name, bg1_screen_data)?;

    Ok(())
}

fn write_screen_data(path: &PathBuf, file_name: &str, screen: &Screen) -> Result<(), ProjectIOError> {
    let screen_location = path.join(file_name);
    let bytes: Vec<u8> = screen.into();
    Ok(fs::write(screen_location, bytes)?)
}

impl TryFrom<PathBuf> for Project {
    type Error = ProjectIOError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let Structure { name, screens } = read_structure(&path)?;
        let background_palette = read_palette(&path, "background_palette.bin")?;
        let object_palette = read_palette(&path, "object_palette.bin")?;
        let object_character_data = read_character_data(&path, "object_character_data.bin")?;
        let screens = screens
            .into_iter()
            .map(|name| read_vram_data(&path, name))
            .collect::<Result<HashMap<String, VRamData>, ProjectIOError>>()?;
        Ok(Project {
            name,
            path,
            background_palette,
            object_palette,
            object_character_data,
            screens,
        })
    }
}

fn read_structure(path: &PathBuf) -> Result<Structure, ProjectIOError> {
    let structure_location = path.join("structure.json");
    let file = File::open(structure_location)?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

fn read_palette(path: &PathBuf, file_name: &str) -> Result<Palette, ProjectIOError> {
    let palette_location = path.join(file_name);
    let file = File::open(palette_location)?;
    Ok(Palette::from(file))
}

fn read_character_data(path: &PathBuf, file_name: &str) -> Result<TileMap, ProjectIOError> {
    let tile_map_location = path.join(file_name);
    let file = File::open(tile_map_location)?;
    Ok(TileMap::from(file))
}

fn read_vram_data(
    path: &PathBuf,
    screen_name: String,
) -> Result<(String, VRamData), ProjectIOError> {
    let bg0_character_data_file_name = format!("bg0_{}_character_data.bin", screen_name);
    let bg1_character_data_file_name = format!("bg1_{}_character_data.bin", screen_name);
    let bg0_screen_data_file_name = format!("bg0_{}_screen_data.bin", screen_name);
    let bg1_screen_data_file_name = format!("bg1_{}_screen_data.bin", screen_name);

    let bg0_character_data = read_character_data(path, &bg0_character_data_file_name)?;
    let bg1_character_data = read_character_data(path, &bg1_character_data_file_name)?;
    let bg0_screen_data = read_screen_data(path, &bg0_screen_data_file_name)?;
    let bg1_screen_data = read_screen_data(path, &bg1_screen_data_file_name)?;

    Ok((
        screen_name,
        VRamData {
            bg0_character_data,
            bg1_character_data,
            bg0_screen_data,
            bg1_screen_data,
        },
    ))
}

fn read_screen_data(path: &PathBuf, file_name: &str) -> Result<Screen, ProjectIOError> {
    let screen_location = path.join(file_name);
    let bytes = fs::read(screen_location)?;
    Ok(Screen::from(bytes))
}

#[cfg(test)]
mod tests {
    use crate::project::Project;
    use std::path::PathBuf;

    #[test]
    fn read_project() {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("resources");
        let project = Project::try_from(directory).unwrap();
        println!("{:?}", project)
    }
}
