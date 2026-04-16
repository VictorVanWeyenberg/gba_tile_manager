use crate::boop::Boops;
use crate::err::ProjectIOError;
use crate::map::CharacterData;
use crate::palette::Palette;
use crate::screen::ScreenData;
use std::default::Default;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub trait Savable: Sized {
    fn name(&self) -> &str;
    fn suffix() -> &'static str;
    fn create<R: Read>(name: impl ToString, data: R) -> Self;
    fn as_data(&self) -> Vec<u8>;
    fn read<P: AsRef<Path> + Debug>(path: P) -> Result<Self, ProjectIOError> {
        let name = if let Some(file_name) = path.as_ref().file_name() {
            let file_name = file_name.to_str().unwrap();
            if file_name.ends_with(Self::suffix()) {
                file_name.replace(Self::suffix(), "")
            } else {
                return Err(format!(
                    "File is not a palette file (ending in `_palette.bin`) `{file_name}`"
                )
                .as_str()
                .into());
            }
        } else {
            return Err(format!("Unable to determine file name of path `{path:?}`")
                .as_str()
                .into());
        };

        let file = File::open(path)?;
        Ok(Self::create(name, file))
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, ProjectIOError> {
        let file_name = format!("{}{}", self.name(), Self::suffix());
        let file_path = path.as_ref().join(file_name);
        let bytes: Vec<u8> = self.as_data();
        fs::write(&file_path, bytes)?;
        Ok(file_path)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Project {
    path: PathBuf,
    palettes: Vec<Palette>,
    character_maps: Vec<CharacterData>,
    screen_maps: Vec<ScreenData>,
    boop_maps: Vec<Boops>,
}

impl Project {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            palettes: Default::default(),
            character_maps: Default::default(),
            screen_maps: Default::default(),
            boop_maps: Default::default(),
        }
    }

    pub fn save(&self) -> Result<(), ProjectIOError> {
        let Project {
            path,
            palettes,
            character_maps,
            screen_maps,
            boop_maps,
        } = self;
        // TODO: Write to temp dir, then move.
        for palette in palettes {
            palette.save(path)?;
        }
        for character_map in character_maps {
            character_map.save(path)?;
        }
        for screen_data in screen_maps {
            screen_data.save(path)?;
        }
        for boop_map in boop_maps {
            boop_map.save(path)?;
        }
        Ok(())
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn palette(&self, name: &str) -> Option<&Palette> {
        self.palettes.iter().find(|palette| palette.name() == name)
    }

    pub fn palette_mut(&mut self, name: &str) -> Option<&mut Palette> {
        self.palettes
            .iter_mut()
            .find(|palette| palette.name() == name)
    }

    pub fn add_palette(&mut self, name: &str) {
        self.palettes.push(Palette::new(name))
    }

    pub fn palette_names(&self) -> Vec<String> {
        self.palettes
            .iter()
            .map(|palette| palette.name().to_string())
            .collect()
    }

    pub fn character_data(&self, name: &str) -> Option<&CharacterData> {
        self.character_maps.iter().find(|map| map.name() == name)
    }

    pub fn character_data_mut(&mut self, name: &str) -> Option<&mut CharacterData> {
        self.character_maps
            .iter_mut()
            .find(|map| map.name() == name)
    }

    pub fn add_character_data(&mut self, name: &str) {
        self.character_maps.push(CharacterData::new(name))
    }

    pub fn character_data_names(&self) -> Vec<String> {
        self.character_maps.iter()
            .map(|map| map.name().to_string())
            .collect()
    }

    pub fn screen_data(&self, name: &str) -> Option<&ScreenData> {
        self.screen_maps.iter().find(|map| map.name() == name)
    }

    pub fn screen_data_mut(&mut self, name: &str) -> Option<&mut ScreenData> {
        self.screen_maps.iter_mut().find(|map| map.name() == name)
    }

    pub fn add_screen_data(&mut self, name: &str) {
        self.screen_maps.push(ScreenData::new(name))
    }

    pub fn screen_names(&self) -> Vec<String> {
        self.screen_maps.iter()
            .map(|screen| screen.name().to_string())
            .collect()
    }

    pub fn boop_data(&self, name: &str) -> Option<&Boops> {
        self.boop_maps.iter().find(|map| map.name() == name)
    }

    pub fn boop_data_mut(&mut self, name: &str) -> Option<&mut Boops> {
        self.boop_maps.iter_mut().find(|map| map.name() == name)
    }

    pub fn add_boop_data(&mut self, name: &str) {
        self.boop_maps.push(Boops::new(name))
    }

    pub fn boop_names(&self) -> Vec<String> {
        self.boop_maps.iter()
            .map(|boop| boop.name().to_string())
            .collect()
    }
}

impl TryFrom<PathBuf> for Project {
    type Error = ProjectIOError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut palettes = vec![];
        let mut character_maps = vec![];
        let mut screen_maps = vec![];
        let mut boop_maps = vec![];

        let paths = fs::read_dir(&path)?;

        for path in paths {
            let path = path
                .or(Err(ProjectIOError::Custom(
                    "Unable to read project file.".to_string(),
                )))?
                .path();
            if let Ok(palette) = Palette::read(&path) {
                palettes.push(palette)
            } else if let Ok(character_data) = CharacterData::read(&path) {
                character_maps.push(character_data)
            } else if let Ok(screen_data) = ScreenData::read(&path) {
                screen_maps.push(screen_data)
            } else if let Ok(boop_map) = Boops::load(&path) {
                boop_maps.push(boop_map)
            }
        }

        palettes.sort_by(|a, b| a.name().cmp(b.name()));
        character_maps.sort_by(|a, b| a.name().cmp(b.name()));
        screen_maps.sort_by(|a, b| a.name().cmp(b.name()));

        Ok(Project {
            path,
            palettes,
            character_maps,
            screen_maps,
            boop_maps,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::project::Project;
    use std::fs;
    use std::path::PathBuf;
    use tempdir::TempDir;

    fn directory() -> PathBuf {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("resources");
        directory
    }

    fn read_project() -> Project {
        Project::try_from(directory()).unwrap()
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
