use crate::character::Character;
use crate::character_data::CharacterData;
use crate::color::Color;
use crate::error::Error;
use crate::palette::Palette;
use crate::screen::ScreenData;
use crate::tile::Tile;
use crate::tile_iter::TiledIterExt;
use image::{DynamicImage, ImageReader};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
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
    image: image::DynamicImage,
    character_maps: Vec<CharacterNode>,
}

impl Debug for PaletteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.name, self.character_maps)
    }
}

impl PaletteNode {
    fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        let image = ImageReader::open(&path)?.decode()?;
        Ok(Self {
            name,
            image,
            character_maps: vec![],
        })
    }

    fn verify(&self) -> Result<(), Error> {
        if self.image.width() != 16 || self.image.height() != 16 {
            return Err(Error::Custom(format!(
                "Palette dimensions off ({}x{}) != (16x16)",
                self.image.width(), self.image.height()
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
        let colors = self.image.to_rgb8()
            .chunks_exact(3)
            .map(|c| Color::new(c[0] / 8, c[1] / 8, c[2] / 8).unwrap())
            .collect();
        Ok(Palette::with_colors(&self.name, colors))
    }
}

pub struct CharacterNode {
    name: String,
    image: DynamicImage,
    screens: Vec<ScreenNode>,
}

impl Debug for CharacterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.name, self.screens)
    }
}

impl CharacterNode {
    fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        let image = ImageReader::open(&path)?.decode()?;
        Ok(Self {
            name,
            image,
            screens: vec![],
        })
    }

    fn verify(&self) -> Result<(), Error> {
        if self.image.width() != 256 || self.image.height() != 256 {
            return Err(Error::Custom(format!(
                "Character data dimensions off ({}x{}) != (256x256)",
                self.image.width(), self.image.height()
            )));
        }
        for screen in &self.screens {
            screen.verify()?
        }
        Ok(())
    }

    fn digest(&mut self, digests: &mut Digests, palette: &Palette) -> Result<(), Error> {
        let character_data = self.as_character_data(palette)?;
        for screen in &mut self.screens {
            screen.digest(digests, palette, &character_data)?;
        }
        digests.characters.push(character_data);
        Ok(())
    }

    fn as_character_data(&mut self, palette: &Palette) -> Result<CharacterData, Error> {
        let buf = self.image.to_rgb8().into_raw();
        let pal_idx = colors_to_palette_index(buf, palette)?;
        let tiles = tiles_from_pal_idx(pal_idx);
        Ok(CharacterData::with_tiles(self.name.clone(), tiles))
    }
}

pub struct ScreenNode {
    name: String,
    image: DynamicImage,
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
            image: ImageReader::open(&path)?.decode()?,
        })
    }

    fn verify(&self) -> Result<(), Error> {
        if self.image.width() != 256 || self.image.height() != 256 {
            return Err(Error::Custom(format!(
                "Screen data dimensions off ({}x{}) != (256x256)",
                self.image.width(), self.image.height()
            )));
        }
        Ok(())
    }

    fn as_screen_data(&mut self, palette: &Palette, character_data: &CharacterData) -> Result<ScreenData, Error> {
        let buf = self.image.to_rgb8().into_raw();
        let pal_idx = colors_to_palette_index(buf, palette)?;
        let tiles = tiles_from_pal_idx(pal_idx);
        let characters = tiles.into_iter()
            .map(|tile| tiles_to_characters(tile, character_data))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(ScreenData::with_characters(&self.name, characters))
    }

    fn digest(&mut self, digests: &mut Digests, palette: &Palette, character_data: &CharacterData) -> Result<(), Error> {
        Ok(digests.screens.push(self.as_screen_data(palette, character_data)?))
    }
}

fn tiles_to_characters(needle: Tile, haystack: &CharacterData) -> Result<Character, Error> {
    for (idx, tile) in haystack.iter().enumerate() {
        if &needle == tile {
            return Ok(Character::new(idx, false, false, 0));
        }
        if &flip_tile(&needle, true, false) == tile {
            return Ok(Character::new(idx, true, false, 0));
        }
        if &flip_tile(&needle, false, true) == tile {
            return Ok(Character::new(idx, false, true, 0));
        }
        if &flip_tile(&needle, true, true) == tile {
            return Ok(Character::new(idx, true, true, 0));
        }
    }
    needle.chunks_exact(8).for_each(|chunk| println!("{chunk:?}"));
    Err(Error::Custom("Tile not found".to_string()))
}

fn flip_tile(tile: &Tile, hflip: bool, vflip: bool) -> Tile {
    let mut result = [0u8; 64];
    for y in 0..8 {
        for x in 0..8 {
            let src_x = if hflip { 7 - x } else { x };
            let src_y = if vflip { 7 - y } else { y };
            result[y * 8 + x] = tile[src_y * 8 + src_x];
        }
    }
    Tile::new(result)
}

fn colors_to_palette_index(rgbs: Vec<u8>, palette: &Palette) -> Result<Vec<u8>, Error> {
    Ok(rgbs
        .chunks_exact(3)
        .map(|c| Color::new(c[0] / 8, c[1] / 8, c[2] / 8).unwrap())
        .map(|c| palette.iter().position(|color| color == &c)
            .map(|idx| idx as u8)
            .ok_or(Error::Custom(format!("Palette color {} not found", c))))
        .collect::<Result<Vec<_>, Error>>()?)
}

fn tiles_from_pal_idx(pal_idx: Vec<u8>) -> Vec<Tile> {
    pal_idx
        .into_iter()
        .tiled()
        .tile_chunked()
        .into_iter()
        .map(|tile_data| Tile::new(tile_data))
        .collect()
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
    use crate::character::Character;
    use crate::character_data::CharacterData;
    use crate::color::Color;
    use crate::palette::Palette;
    use crate::project::{CharacterNode, PaletteNode, Project, ScreenNode};
    use crate::screen::ScreenData;
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

    fn read_palette() -> Palette {
        let palette =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/background_palette.png");
        let mut palette = PaletteNode::new("background_palette.png".into(), palette)
            .expect("Could not create palette");
        palette.as_palette().unwrap()
    }

    fn read_character_data(file_name: &str, palette: &Palette) -> CharacterData {
        let character_data =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&format!("resources/{file_name}"));
        let mut character_data = CharacterNode::new(file_name.to_string(), character_data)
            .expect("Could not create character data");
        character_data.as_character_data(palette).unwrap()
    }

    fn read_screen_data(palette: &Palette, character_data: &CharacterData) -> ScreenData {
        let screen_data =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/empty_art_bg0.png");
        let mut screen_data = ScreenNode::new("empty_art_bg0.png".to_string(), screen_data)
            .expect("Could not create screen data");
        screen_data.as_screen_data(palette, character_data).unwrap()
    }

    #[test]
    fn palette_from_image() {
        let palette = read_palette();
        assert_eq!(palette.len(), 6);
        assert_eq!(palette.get(0), Some(&Color::new(5, 6, 6).unwrap()));
        assert_eq!(palette.get(1), Some(&Color::new(9, 9, 13).unwrap()));
        assert_eq!(palette.get(2), Some(&Color::new(14, 13, 16).unwrap()));
        assert_eq!(palette.get(3), Some(&Color::new(27, 26, 27).unwrap()));
        assert_eq!(palette.get(4), Some(&Color::new(2, 5, 13).unwrap()));
        assert_eq!(palette.get(5), Some(&Color::new(21, 23, 21).unwrap()));
    }

    #[test]
    fn character_data_from_image() {
        let palette = read_palette();
        let character_data = read_character_data("bg1_empty_art.png", &palette);

        assert_eq!(character_data.len(), 100);
        let tile = character_data.get(14).unwrap();
        for idx in 0..63 {
            if idx == 49 || idx == 50 || idx == 41 || idx == 42 {
                assert_eq!(tile[idx], 3, "{idx}");
            } else {
                assert_eq!(tile[idx], 0, "{idx}");
            }
        }
    }

    #[test]
    fn screen_data_from_image() {
        let palette = read_palette();
        let character_data = read_character_data("bg0_empty_art.png", &palette);
        let screen_data = read_screen_data(&palette, &character_data);
        assert_eq!(screen_data.len(), 1024);
        let top_left = screen_data.get(0).unwrap();
        let top_right = screen_data.get(29).unwrap();
        let bottom_left = screen_data.get(19*32).unwrap();
        let bottom_right = screen_data.get(19*32+29).unwrap();

        assert_eq!(&Character {
            tile_number: 1,
            horizontal_flip: false,
            vertical_flip: false,
            palette_number: 0,
        }, top_left);
        assert_eq!(&Character {
            tile_number: 1,
            horizontal_flip: true,
            vertical_flip: false,
            palette_number: 0,
        }, top_right);
        assert_eq!(&Character {
            tile_number: 1,
            horizontal_flip: false,
            vertical_flip: true,
            palette_number: 0,
        }, bottom_left);
        assert_eq!(&Character {
            tile_number: 1,
            horizontal_flip: true,
            vertical_flip: true,
            palette_number: 0,
        }, bottom_right);
    }
}
