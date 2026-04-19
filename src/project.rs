use crate::character_data::CharacterData;
use crate::color::Color;
use crate::error::Error;
use crate::palette::Palette;
use crate::screen::ScreenData;
use crate::tile_iter::TiledIterExt;
use png::Decoder;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::png_util::read_to_rgb_255;
use crate::tile::Tile;

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

#[derive(Default)]
pub struct Digests {
    palettes: Vec<Palette>,
    characters: Vec<CharacterData>,
    screens: Vec<ScreenData>,
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

    pub fn digest(&mut self) -> Result<Digests, Error> {
        let mut digest = Digests::default();
        for palette in &mut self.palettes {
            palette.digest(&mut digest)?;
        }
        Ok(digest)
    }
}

pub struct PaletteNode {
    name: String,
    reader: png::Reader<BufReader<File>>,
    character_maps: Vec<CharacterNode>,
}

impl Debug for PaletteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.name, self.character_maps)
    }
}

impl PaletteNode {
    fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        let reader = Decoder::new(BufReader::new(File::open(path)?)).read_info()?;
        println!("{}, {:#?}", name, reader.info().color_type);
        Ok(Self {
            name,
            reader,
            character_maps: vec![],
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
        for character_map in &self.character_maps {
            character_map.verify()?;
        }
        Ok(())
    }

    fn digest(&mut self, digests: &mut Digests) -> Result<(), Error> {
        let palette = self.as_palette()?;
        for character_map in &mut self.character_maps {
            character_map.digest(digests, &palette)?;
        }
        digests.palettes.push(palette);
        Ok(())
    }

    fn as_palette(&mut self) -> Result<Palette, Error> {
        let buf = read_to_rgb_255(&mut self.reader)?;
        let colors = buf
            .chunks_exact(3)
            .map(|c| Color::new(c[0] / 8, c[1] / 8, c[2] / 8).unwrap())
            .collect();
        Ok(Palette::with_colors(&self.name, colors))
    }
}

pub struct CharacterNode {
    name: String,
    reader: png::Reader<BufReader<File>>,
    screens: Vec<ScreenNode>,
}

impl Debug for CharacterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.name, self.screens)
    }
}

impl CharacterNode {
    fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        let reader = Decoder::new(BufReader::new(File::open(path)?)).read_info()?;
        println!("{}, {:#?}", name, reader.info().color_type);
        Ok(Self {
            name,
            reader,
            screens: vec![],
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
        for screen in &self.screens {
            screen.verify()?
        }
        Ok(())
    }

    fn digest(&mut self, digests: &mut Digests, palette: &Palette) -> Result<(), Error> {
        let character_data = self.as_character_data(palette)?;
        for screen in &self.screens {
            screen.digest(digests)?;
        }
        digests.characters.push(character_data);
        Ok(())
    }

    fn as_character_data(&mut self, palette: &Palette) -> Result<CharacterData, Error> {
        let buf = read_to_rgb_255(&mut self.reader)?;
        let tiles = buf
            .chunks_exact(3)
            .map(|c| {
                Color::new(c[0] / 8, c[1] / 8, c[2] / 8).unwrap()
            })
            .map(|c| palette.iter().position(|color| color == &c)
                .map(|idx| idx as u8)
                .ok_or(Error::Custom(format!("Palette color {} not found", c))))
            .collect::<Result<Vec<_>, Error>>()?
            .into_iter()
            .tiled()
            .tile_chunked()
            .into_iter()
            .map(|tile_data| Tile::new(tile_data))
            .collect();
        Ok(CharacterData::with_tiles(self.name.clone(), tiles))
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

    fn digest(&self, digests: &mut Digests) -> Result<(), Error> {
        Ok(digests.screens.push((&self.reader).try_into()?))
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
                let mut character =
                    CharacterNode::new(character.clone(), directory.join(character))?;
                for screen in screens {
                    character
                        .screens
                        .push(ScreenNode::new(screen.clone(), directory.join(screen))?);
                }
                palette.character_maps.push(character);
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
    use crate::color::Color;
    use crate::project::{PaletteNode, Project};
    use std::path::PathBuf;

    #[test]
    fn read_project() {
        let mut project: Project = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .try_into()
            .expect("Could not open project");
        println!("{project:?}");
        project.digest().expect("Could not digest project");
    }

    #[test]
    fn palette_from_image() {
        let palette =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/background_palette.png");
        let mut palette = PaletteNode::new("background_palette.png".into(), palette)
            .expect("Could not create palette");
        let palette = palette.as_palette().unwrap();
        assert_eq!(palette.len(), 6);
        assert_eq!(palette.get(0), Some(&Color::new(5, 6, 6).unwrap()));
        assert_eq!(palette.get(1), Some(&Color::new(9, 9, 13).unwrap()));
        assert_eq!(palette.get(2), Some(&Color::new(14, 13, 16).unwrap()));
        assert_eq!(palette.get(3), Some(&Color::new(27, 26, 27).unwrap()));
        assert_eq!(palette.get(4), Some(&Color::new(2, 5, 13).unwrap()));
        assert_eq!(palette.get(5), Some(&Color::new(21, 23, 21).unwrap()));
    }
}
