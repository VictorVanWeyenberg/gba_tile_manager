use crate::character::Character;
use crate::project::Savable;
use std::io::Read;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ScreenData {
    name: String,
    characters: [[Character; 32]; 32],
}

impl ScreenData {
    pub fn get_character(&self, x: usize, y: usize) -> &Character {
        &self.characters[y][x]
    }

    pub fn set_character(&mut self, character: Character, x: usize, y: usize) {
        self.characters[y][x] = character
    }
}

impl Savable for ScreenData {
    fn name(&self) -> &str {
        &self.name
    }

    fn suffix() -> &'static str {
        "_screen_data.bin"
    }

    fn create<R: Read>(name: impl ToString, mut data: R) -> Self {
        let mut buf = [0u8; 2];
        let mut characters = [[Character::default(); 32]; 32];
        let mut index = 0;
        while data.read_exact(&mut buf).is_ok() {
            let x = index % 32;
            let y = index / 32;
            characters[y][x] = Character::from(buf);
            index += 1;
        }
        ScreenData { name: name.to_string(), characters }
    }

    fn as_data(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.characters
            .into_iter()
            .flatten()
            .map::<[u8; 2], _>(Character::into)
            .flatten()
            .collect();

        bytes.chunks_exact(2)
            .rposition(|b| b[0] != 0 || b[1] != 0)
            .map_or(&[][..], |i| &bytes[..=i * 2 + 1])
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use tempdir::TempDir;
    use crate::character::Character;
    use crate::project::Savable;
    use crate::screen::ScreenData;

    #[test]
    fn screen_round_trip() {
        let temp_dir = TempDir::new("gba_tile_manager::screen_round_trip")
            .unwrap()
            .path()
            .to_owned();
        fs::create_dir(temp_dir.clone()).unwrap();

        let mut screen = ScreenData::default();
        screen.set_character(Character::new(0, false, false, 0), 0, 0);
        screen.set_character(Character::new(1, false, false, 1), 1, 0);
        screen.set_character(Character::new(2, false, false, 2), 2, 0);
        screen.set_character(Character::new(3, false, false, 3), 3, 0);
        screen.set_character(Character::new(4, false, false, 4), 4, 0);
        screen.set_character(Character::new(5, false, false, 5), 0, 1);

        let screen_data_path = screen.save(temp_dir).expect("Could not save screen data.");
        let screen = ScreenData::read(screen_data_path).expect("Could not read screen data.");

        assert_eq!(screen.get_character(0, 0), &Character::new(0, false, false, 0));
        assert_eq!(screen.get_character(1, 0), &Character::new(1, false, false, 1));
        assert_eq!(screen.get_character(2, 0), &Character::new(2, false, false, 2));
        assert_eq!(screen.get_character(3, 0), &Character::new(3, false, false, 3));
        assert_eq!(screen.get_character(4, 0), &Character::new(4, false, false, 4));
        assert_eq!(screen.get_character(0, 1), &Character::new(5, false, false, 5));

        for x in 0..30 {
            for y in 0..20 {
                match (x, y) {
                    (x, 0) if x < 5 => continue,
                    (0, 1) => continue,
                    _ => assert_eq!(screen.get_character(x, y), &Character::new(0, false, false, 0)),
                }
            }
        }
    }
}