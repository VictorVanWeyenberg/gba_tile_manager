use crate::project::Project;
use crate::ui::{Message, TilesState};
use iced::widget::{button, column, combo_box, row, text_input};
use iced::{Element, Length};
use crate::ui::palette_selector::palette_selector;
use crate::ui::tile_selector::tile_selector;

pub fn character_map_selector<'a>(
    project: &'a Project,
    tiles_state @ TilesState {
        palette_name,
        character_data_name,
        character_data_names,
        palettes_names,
        new_character_map_name,
        ..
    }: &'a TilesState,
) -> Element<'a, Message> {
    let mut input = column!(
        row![
            text_input("Character Map Name", &new_character_map_name).width(Length::FillPortion(5)),
            button("Add")
                .on_press(Message::AddCharacterMap)
                .width(Length::FillPortion(1))
        ],
        combo_box(
            character_data_names,
            "Pick character map...",
            character_data_name.into(),
            Message::CharacterMapSelected
        ),
        combo_box(
            palettes_names,
            "Pick render palette...",
            palette_name.into(),
            Message::TilesRenderPaletteSelected
        ),
    );
    if let Some(selector) = tile_selector(project, tiles_state) {
        input = input.push(selector);
    }
    let input = input.spacing(10).padding(10);
    let mut input = row![
        input,
    ];
    if let Some(selector) = palette_selector(project, tiles_state) {
        input = input.push(selector);
    }
    input.spacing(10).padding(10).into()
}
