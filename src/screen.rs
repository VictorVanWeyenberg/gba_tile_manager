use crate::character::Character;
use crate::savable::Savable;
use std::io::Read;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq)]
pub struct ScreenData {
    name: String,
    characters: Vec<Character>,
}

impl ScreenData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            characters: vec![Character::default(); 32 * 32],
        }
    }
}

impl Deref for ScreenData {
    type Target = Vec<Character>;
    fn deref(&self) -> &Self::Target {
        &self.characters
    }
}

impl DerefMut for ScreenData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.characters
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
        let mut characters = vec![];
        while data.read_exact(&mut buf).is_ok() {
            characters.push(Character::from(buf));
        }
        while characters.len() < 32 * 32 {
            characters.push(Character::default());
        }
        ScreenData {
            name: name.to_string(),
            characters,
        }
    }

    fn as_data(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self
            .characters
            .iter()
            .map::<[u8; 2], _>(|character|character.into())
            .flatten()
            .collect();

        bytes
            .chunks_exact(2)
            .rposition(|b| b[0] != 0 || b[1] != 0)
            .map_or(&[][..], |i| &bytes[..=i * 2 + 1])
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::character::Character;
    use crate::savable::Savable;
    use crate::screen::ScreenData;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn screen_round_trip() {
        let temp_dir = TempDir::new("gba_tile_manager::screen_round_trip")
            .unwrap()
            .path()
            .to_owned();
        fs::create_dir(temp_dir.clone()).unwrap();

        let mut screen = ScreenData::new("Test screen");
        screen[0] = Character::new(0, false, false, 0);
        screen[1] = Character::new(1, false, false, 1);
        screen[2] = Character::new(2, false, false, 2);
        screen[3] = Character::new(3, false, false, 3);
        screen[4] = Character::new(4, false, false, 4);
        screen[33] = Character::new(5, false, false, 5);

        let screen_data_path = screen.save(temp_dir).expect("Could not save screen data.");
        let screen = ScreenData::read(screen_data_path).expect("Could not read screen data.");

        assert_eq!(
            &screen[0],
            &Character::new(0, false, false, 0)
        );
        assert_eq!(
            &screen[1],
            &Character::new(1, false, false, 1)
        );
        assert_eq!(
            &screen[2],
            &Character::new(2, false, false, 2)
        );
        assert_eq!(
            &screen[3],
            &Character::new(3, false, false, 3)
        );
        assert_eq!(
            &screen[4],
            &Character::new(4, false, false, 4)
        );
        assert_eq!(
            &screen[33],
            &Character::new(5, false, false, 5)
        );

        for idx in 0..32*32 {
            if idx < 5 || idx == 33 {
                continue
            }
            assert_eq!(
                &screen[idx],
                &Character::new(0, false, false, 0)
            )
        }
    }
}
