use crate::error::Error;
use png::Decoder;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Config {
    name: String,
    screens: Vec<ScreenConfig>,
}

#[derive(Deserialize)]
struct ScreenConfig {
    palette: String,
    character: String,
    screen: String,
}

#[derive(Debug)]
pub struct Project {
    name: String,
    palettes: Vec<PaletteNode>,
}

impl Project {
    pub fn name(&self) -> &str {
        &self.name
    }
    
    fn verify(&self) -> Result<(), Error> {
        for palettes in &self.palettes {
            palettes.verify()?;
        }
        Ok(())
    }
}

pub struct PaletteNode {
    name: String,
    reader: png::Reader<BufReader<File>>,
    character: Vec<CharacterNode>,
}

impl Debug for PaletteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.name, self.character)
    }
}

impl PaletteNode {
    fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            name,
            reader: Decoder::new(BufReader::new(File::open(path)?)).read_info()?,
            character: vec![],
        })
    }

    fn verify(&self) -> Result<(), Error> {
        let info = self.reader.info();
        if info.width != 16 || info.height != 16 {
            return Err(Error::Custom(format!(
                "Palette dimensions off ({}x{}) != (16x16)",
                info.width, info.height
            )));
        }
        for character in &self.character {
            character.verify()?;
        }
        Ok(())
    }
}

pub struct CharacterNode {
    name: String,
    reader: png::Reader<BufReader<File>>,
    screen: Vec<ScreenNode>,
}

impl Debug for CharacterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.name, self.screen)
    }
}

impl CharacterNode {
    fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            name,
            reader: Decoder::new(BufReader::new(File::open(path)?)).read_info()?,
            screen: vec![],
        })
    }

    fn verify(&self) -> Result<(), Error> {
        let info = self.reader.info();
        if info.width != 256 || info.height != 256 {
            return Err(Error::Custom(format!(
                "Character data dimensions off ({}x{}) != (256x256)",
                info.width, info.height
            )));
        }
        for screen in &self.screen {
            screen.verify()?
        }
        Ok(())
    }
}

pub struct ScreenNode {
    name: String,
    reader: png::Reader<BufReader<File>>,
}

impl Debug for ScreenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ScreenNode {
    fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            name,
            reader: Decoder::new(BufReader::new(File::open(path)?)).read_info()?,
        })
    }

    fn verify(&self) -> Result<(), Error> {
        let info = self.reader.info();
        if info.width != 256 || info.height != 256 {
            return Err(Error::Custom(format!(
                "Screen data dimensions off ({}x{}) != (256x256)",
                info.width, info.height
            )));
        }
        Ok(())
    }
}

impl TryFrom<PathBuf> for Project {
    type Error = Error;

    fn try_from(directory: PathBuf) -> Result<Self, Self::Error> {
        let config_path = directory.join("config.json");
        let config: Config = serde_json::from_reader(File::open(config_path)?)?;
        let name = config.name;
        let mut screens: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
        for screen in config.screens {
            screens
                .entry(screen.palette)
                .or_insert(HashMap::new())
                .entry(screen.character)
                .or_insert(Vec::new())
                .push(screen.screen);
        }
        let mut palettes = vec![];
        for (palette, characters) in screens {
            let mut palette = PaletteNode::new(palette.clone(), directory.join(palette))?;
            for (character, screens) in characters {
                let mut character = CharacterNode::new(character.clone(), directory.join(character))?;
                for screen in screens {
                    character
                        .screen
                        .push(ScreenNode::new(screen.clone(), directory.join(screen))?);
                }
                palette.character.push(character);
            }
            palettes.push(palette);
        }
        let project = Project { name, palettes };
        project.verify()?;
        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::project::Project;

    #[test]
    fn read_project() {
        let project: Project = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .try_into()
            .expect("Could not open project");
        println!("{project:?}");
    }
}
