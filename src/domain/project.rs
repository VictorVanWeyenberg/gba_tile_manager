use crate::err::ProjectIOError;
use crate::map::CharacterData;
use crate::palette::Palette;
use crate::screen::ScreenData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Structure {
    name: String,
    #[serde(default)]
    palettes: Vec<String>,
    #[serde(default)]
    character_maps: Vec<CharacterMapStructure>,
    #[serde(default)]
    screen_maps: Vec<ScreenMapStructure>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CharacterMapStructure {
    name: String,
    render_palette: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ScreenMapStructure {
    name: String,
    render_palette: String,
    render_character_map: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Project {
    name: String,
    path: PathBuf,
    palettes: HashMap<String, Palette>,
    character_maps: HashMap<CharacterMapStructure, CharacterData>,
    screen_maps: HashMap<ScreenMapStructure, ScreenData>,
}

impl Project {
    pub fn new(name: impl ToString, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            path,
            palettes: Default::default(),
            character_maps: Default::default(),
            screen_maps: Default::default(),
        }
    }

    pub fn save(&self) -> Result<(), ProjectIOError> {
        let Project {
            name, path, palettes, character_maps, screen_maps
        } = self;
        // TODO: Write to temp dir, then move.
        write_structure(
            path,
            Structure {
                name: name.to_string(),
                palettes: palettes.keys().cloned().collect(),
                character_maps: character_maps.keys().cloned().collect(),
                screen_maps: screen_maps.keys().cloned().collect(),
            },
        )?;
        for (name, palette) in palettes {
            write_palette(path, &format!("{name}_palette.bin"), palette)?;
        }
        for (CharacterMapStructure { name, .. }, character_map) in character_maps {
            write_character_data(path, &format!("{name}_character_data.bin"), character_map)?;
        }
        for (ScreenMapStructure { name, .. }, screen_data) in screen_maps {
            write_screen_data(path, &format!("{name}_screen_data.bin"), screen_data)?;
        }
        Ok(())
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }


    pub fn palette(&self, name: &str) -> Option<&Palette> {
        self.palettes.get(name)
    }

    pub fn palette_mut(&mut self, name: &str) -> Option<&mut Palette> {
        self.palettes.get_mut(name)
    }

    pub fn add_palette(&mut self, name: &str) {
        self.palettes.insert(name.to_string(), Palette::default());
    }

    pub fn palette_names(&self) -> Vec<&String> {
        self.palettes.keys().collect()
    }

    pub fn character_data(&self, name: &str) -> Option<(&CharacterMapStructure, &CharacterData)> {
        self.character_maps.iter()
            .find(|(key, _)| {
                key.name == name
            })
    }

    pub fn character_data_mut(&mut self, name: &str) -> Option<(&CharacterMapStructure, &mut CharacterData)> {
        self.character_maps.iter_mut()
            .find(|(key, _)| {
                key.name == name
            })
    }

    pub fn screen_data(&self, name: &str) -> Option<(&ScreenMapStructure, &ScreenData)> {
        self.screen_maps.iter()
            .find(|(key, _)| {
                key.name == name
            })
    }

    pub fn screen_data_mut(&mut self, name: &str) -> Option<(&ScreenMapStructure, &mut ScreenData)> {
        self.screen_maps.iter_mut()
            .find(|(key, _)| {
                key.name == name
            })
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
    tile_map: &CharacterData,
) -> Result<(), ProjectIOError> {
    let character_data_location = path.join(file_name);
    let bytes: Vec<u8> = tile_map.into();
    Ok(fs::write(character_data_location, bytes)?)
}

fn write_screen_data(
    path: &PathBuf,
    file_name: &str,
    screen: &ScreenData,
) -> Result<(), ProjectIOError> {
    let screen_location = path.join(file_name);
    let bytes: Vec<u8> = screen.into();
    Ok(fs::write(screen_location, bytes)?)
}

impl TryFrom<PathBuf> for Project {
    type Error = ProjectIOError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let Structure { name, palettes, character_maps, screen_maps, } = read_structure(&path)?;
        let palettes = palettes.into_iter()
            .map(|name| read_palette(&path, name))
            .collect::<Result<HashMap<String, Palette>, ProjectIOError>>()?;
        let character_maps = character_maps.into_iter()
            .map(|structure| read_character_data(&path, structure))
            .collect::<Result<HashMap<CharacterMapStructure, CharacterData>, ProjectIOError>>()?;
        let screen_maps = screen_maps.into_iter()
            .map(|structure| read_screen_data(&path, structure))
            .collect::<Result<HashMap<ScreenMapStructure, ScreenData>, ProjectIOError>>()?;
        Ok(Project {
            name,
            path,
            palettes,
            character_maps,
            screen_maps,
        })
    }
}

fn read_structure(path: &PathBuf) -> Result<Structure, ProjectIOError> {
    let structure_location = path.join("structure.json");
    let file = File::open(structure_location)?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

fn read_palette(path: &PathBuf, name: String) -> Result<(String, Palette), ProjectIOError> {
    let file_name = format!("{name}_palette.bin");
    let palette_location = path.join(file_name);
    let file = File::open(palette_location)?;
    Ok((name, Palette::from(file)))
}

fn read_character_data(path: &PathBuf, character_map_structure: CharacterMapStructure) -> Result<(CharacterMapStructure, CharacterData), ProjectIOError> {
    let file_name = &format!("{}_character_data.bin", &character_map_structure.name);
    let tile_map_location = path.join(file_name);
    let file = File::open(tile_map_location)?;
    Ok((character_map_structure, CharacterData::from(file)))
}

fn read_screen_data(path: &PathBuf, screen_map_structure: ScreenMapStructure) -> Result<(ScreenMapStructure, ScreenData), ProjectIOError> {
    let file_name = format!("{}_screen_data.bin", &screen_map_structure.name);
    let screen_location = path.join(file_name);
    let bytes = fs::read(screen_location)?;
    Ok((screen_map_structure, ScreenData::from(bytes)))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use tempdir::TempDir;
    use crate::project::Project;

    fn read_project() -> Project {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("resources");
        Project::try_from(directory).unwrap()
    }

    #[test]
    fn project_round_trip() {
        let temp_dir = TempDir::new("gba_tile_manager::project::tests::project_round_trip")
            .unwrap()
            .path()
            .to_owned();
        fs::create_dir(temp_dir.clone()).unwrap();

        let mut this = read_project();
        this.path = temp_dir.clone();
        this.save().unwrap();

        let that = Project::try_from(temp_dir).unwrap();

        assert_eq!(this, that);
    }
}
