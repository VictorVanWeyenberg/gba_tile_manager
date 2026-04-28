use crate::character::Character;
use crate::character_data::CharacterData;
use crate::color::Color;
use crate::error::Error;
use crate::palette::Palette;
use crate::project::boop::BoopNode;
use crate::project::character::CharacterNode;
use crate::project::digest::Digests;
use crate::project::palette::PaletteNode;
use crate::project::screen::ScreenNode;
use crate::savable::Savable;
use crate::tile::Tile;
use crate::tile_iter::TiledIterExt;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::path::{Path, PathBuf};

mod boop;
mod character;
mod csv;
mod digest;
mod palette;
mod screen;

pub use csv::*;

#[derive(Deserialize)]
struct Config {
    name: String,
    #[serde(default)]
    screens: Vec<ScreenConfig>,
    #[serde(default)]
    boops: Vec<String>,
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
    boops: Vec<BoopNode>,
}

impl Project {
    pub fn name(&self) -> &str {
        &self.name
    }

    fn verify(&mut self) -> Result<(), Error> {
        for palettes in &self.palettes {
            palettes.verify()?;
        }
        for boop in &mut self.boops {
            boop.verify()?;
        }
        Ok(())
    }

    pub fn digest(&mut self) -> Result<Digests, Error> {
        let mut digest = Digests::default();
        for palette in &mut self.palettes {
            palette.digest(&mut digest)?;
        }
        for boop in &mut self.boops {
            boop.digest(&mut digest)?;
        }
        Ok(digest)
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
    let text_tile = needle
        .chunks_exact(8)
        .map(|chunk| format!("{chunk:?}"))
        .reduce(|a, b| format!("{a:?}\n{b:?}"))
        .unwrap_or_else(|| String::from(""));
    let character_data_name = haystack.name();
    Err(Error::Custom(format!(
        "Tile not found in character data {character_data_name}\
        \
        {text_tile}"
    )))
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
    rgbs
        .chunks_exact(3)
        .map(|c| Color::new(c[0] / 8, c[1] / 8, c[2] / 8).unwrap())
        .map(|c| {
            palette
                .iter()
                .position(|color| color == &c)
                .map(|idx| idx as u8)
                .ok_or(Error::Custom(format!("Palette color {} not found", c)))
        })
        .collect::<Result<Vec<_>, Error>>()
}

fn tiles_from_pal_idx(pal_idx: Vec<u8>) -> Vec<Tile> {
    pal_idx
        .into_iter()
        .tiled()
        .tile_chunked()
        .into_iter()
        .map(Tile::new)
        .collect()
}

fn verify_is_png_get_file_name(file_name: String) -> Result<String, Error> {
    if file_name.ends_with(".png") {
        Ok(file_name.replace(".png", ""))
    } else {
        Err(Error::Custom(format!(
            "File `{file_name}` is not a png file."
        )))
    }
}

fn determine_boop_file(directory: &Path, file_name: String) -> Result<BoopNode, Error> {
    if !file_name.ends_with(".csv") {
        return Err(Error::Custom(format!(
            "Expected boops file to be .csv. {file_name}"
        )));
    }

    let name = file_name.replace(".csv", "");
    let file = directory.join(file_name.clone());
    let file = File::open(file).map_err(|e| Error::IO(e, file_name))?;
    Ok(BoopNode::new(name, file))
}

impl TryFrom<PathBuf> for Project {
    type Error = Error;

    fn try_from(directory: PathBuf) -> Result<Self, Self::Error> {
        let config_path = directory.join("config.json");
        let config: Config = serde_json::from_reader(
            File::open(config_path.clone())
                .map_err(|e| Error::IO(e, config_path.to_str().unwrap().to_string()))?,
        )?;
        let screens = screens_to_dep_graph(config.screens)?;
        let palettes = dep_graph_to_nodes(&directory, screens)?;
        let boops = config
            .boops
            .into_iter()
            .map(|boop| determine_boop_file(&directory, boop))
            .collect::<Result<Vec<_>, Error>>()?;
        let mut project = Project {
            name: config.name,
            palettes,
            boops,
        };
        project.verify()?;
        Ok(project)
    }
}

fn screens_to_dep_graph(
    screen_configs: Vec<ScreenConfig>,
) -> Result<HashMap<String, HashMap<String, Vec<String>>>, Error> {
    let mut screens = HashMap::new();
    for screen in screen_configs {
        screens
            .entry(verify_is_png_get_file_name(screen.palette)?)
            .or_insert(HashMap::new())
            .entry(verify_is_png_get_file_name(screen.character)?)
            .or_insert(Vec::new())
            .push(verify_is_png_get_file_name(screen.screen)?);
    }
    Ok(screens)
}

fn dep_graph_to_nodes(
    directory: &Path,
    graph: HashMap<String, HashMap<String, Vec<String>>>,
) -> Result<Vec<PaletteNode>, Error> {
    let mut palettes = vec![];
    for (palette, characters) in graph {
        let path = directory.join(format!("{palette}.png"));
        let mut palette = PaletteNode::new(palette, path)?;
        for (character, screens) in characters {
            let path = directory.join(format!("{character}.png"));
            let mut character = CharacterNode::new(character, path)?;
            for screen in screens {
                let path = directory.join(format!("{screen}.png"));
                character.screens_mut().push(ScreenNode::new(screen, path)?);
            }
            palette.character_maps_mut().push(character);
        }
        palettes.push(palette);
    }
    Ok(palettes)
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
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("resources/{file_name}"));
        let mut character_data = CharacterNode::new(file_name.to_string(), character_data)
            .expect("Could not create character data");
        character_data.as_character_data(palette).unwrap()
    }

    fn read_screen_data(palette: &Palette, character_data: &CharacterData) -> ScreenData {
        let screen_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/empty_art/bg0/screen.png");
        let mut screen_data = ScreenNode::new("screen.png".to_string(), screen_data)
            .expect("Could not create screen data");
        screen_data.as_screen_data(palette, character_data).unwrap()
    }

    #[test]
    fn palette_from_image() {
        let palette = read_palette();
        assert_eq!(palette.len(), 6);
        assert_eq!(palette.first(), Some(&Color::new(5, 6, 6).unwrap()));
        assert_eq!(palette.get(1), Some(&Color::new(9, 9, 13).unwrap()));
        assert_eq!(palette.get(2), Some(&Color::new(14, 13, 16).unwrap()));
        assert_eq!(palette.get(3), Some(&Color::new(27, 26, 27).unwrap()));
        assert_eq!(palette.get(4), Some(&Color::new(2, 5, 13).unwrap()));
        assert_eq!(palette.get(5), Some(&Color::new(21, 23, 21).unwrap()));
    }

    #[test]
    fn character_data_from_image() {
        let palette = read_palette();
        let character_data = read_character_data("empty_art/bg1/characters.png", &palette);

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
        let character_data = read_character_data("empty_art/bg0/characters.png", &palette);
        let screen_data = read_screen_data(&palette, &character_data);
        assert_eq!(screen_data.len(), 32 * 20 - 2);
        let top_left = screen_data.first().unwrap();
        let top_right = screen_data.get(29).unwrap();
        let bottom_left = screen_data.get(19 * 32).unwrap();
        let bottom_right = screen_data.get(19 * 32 + 29).unwrap();

        assert_eq!(
            &Character {
                tile_number: 1,
                horizontal_flip: false,
                vertical_flip: false,
                palette_number: 0,
            },
            top_left
        );
        assert_eq!(
            &Character {
                tile_number: 1,
                horizontal_flip: true,
                vertical_flip: false,
                palette_number: 0,
            },
            top_right
        );
        assert_eq!(
            &Character {
                tile_number: 1,
                horizontal_flip: false,
                vertical_flip: true,
                palette_number: 0,
            },
            bottom_left
        );
        assert_eq!(
            &Character {
                tile_number: 1,
                horizontal_flip: true,
                vertical_flip: true,
                palette_number: 0,
            },
            bottom_right
        );
    }
}
