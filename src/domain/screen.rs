use crate::character::Character;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Screen {
    characters: [[Character; 32]; 32],
}

impl Screen {
    pub fn get_character(&self, x: usize, y: usize) -> &Character {
        &self.characters[y][x]
    }

    pub fn set_character(&mut self, character: Character, x: usize, y: usize) {
        self.characters[y][x] = character
    }
}

impl Into<Vec<u8>> for &Screen {
    fn into(self) -> Vec<u8> {
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

impl From<Vec<u8>> for Screen {
    fn from(value: Vec<u8>) -> Self {
        let mut flat = value.chunks_exact(2)
            .map(|chunk| Character::from([chunk[0], chunk[1]]));

        let characters = std::array::from_fn(|_| {
            std::array::from_fn(|_| flat.next().unwrap_or_default())
        });

        Screen { characters }
    }
}

#[cfg(test)]
mod tests {
    use crate::character::Character;
    use crate::screen::Screen;

    #[test]
    fn screen_round_trip() {
        let mut screen = Screen::default();
        screen.set_character(Character::new(0, false, false, 0), 0, 0);
        screen.set_character(Character::new(1, false, false, 1), 1, 0);
        screen.set_character(Character::new(2, false, false, 2), 2, 0);
        screen.set_character(Character::new(3, false, false, 3), 3, 0);
        screen.set_character(Character::new(4, false, false, 4), 4, 0);
        screen.set_character(Character::new(5, false, false, 5), 0, 1);

        let bytes: Vec<u8> = (&screen).into();
        assert_eq!(bytes.len(), 66);
        assert_eq!(bytes[..10], [0x00, 0x00, 0x01, 0x10, 0x02, 0x20, 0x03, 0x30, 0x04, 0x40]);

        let screen = Screen::from(bytes);

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