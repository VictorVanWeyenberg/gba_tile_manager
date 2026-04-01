use crate::project::Project;
use crate::ui::{Message, TilesState};
use iced::widget::{button, column, combo_box, row, text_input};
use iced::{Element, Length};

pub fn character_map_selector<'a>(
    project: &'a Project,
    TilesState {
        palette_name,
        character_data_name,
        selected_tile,
        character_data_names,
        palettes_names,
        location,
        new_character_map_name,
    }: &'a TilesState,
) -> Element<'a, Message> {
    column!(
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
    )
    .width(Length::Fixed(200f32))
    .padding(10)
    .into()
}
