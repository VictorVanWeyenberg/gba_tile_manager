use crate::project::Project;
use crate::ui::editor::editor;
use crate::ui::selector::selector;
use crate::ui::{Message, TileMessage, TilesState};
use iced::widget::{button, column, combo_box, container, row, space, text_input};
use iced::{Element, Length};

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
    row![
        column!(
            row![
                text_input("Character Map Name", &new_character_map_name)
                    .on_input(|name| Message::Tile(TileMessage::NewCharacterMapNameChanged(name)))
                    .width(Length::FillPortion(3)),
                button("Add")
                    .on_press(Message::Tile(TileMessage::AddCharacterMap))
                    .width(Length::FillPortion(1))
            ]
            .spacing(10),
            combo_box(
                character_data_names,
                "Pick character map...",
                character_data_name.into(),
                |name| Message::Tile(TileMessage::CharacterMapSelected(name)),
            ),
            combo_box(
                palettes_names,
                "Pick render palette...",
                palette_name.into(),
                |name| Message::Tile(TileMessage::PaletteSelected(name)),
            ),
            row!(
                button("+").on_press(Message::Tile(TileMessage::AddTile)),
                button("-").on_press(Message::Tile(TileMessage::RemoveTile)),
                button("^").on_press(Message::Tile(TileMessage::MoveTileUp)),
                button("v").on_press(Message::Tile(TileMessage::MoveTileDown))
            )
            .spacing(10),
            tile_selector(project, tiles_state)
        )
        .spacing(10)
        .padding(10)
        .width(Length::FillPortion(5)),
        container(palette_selector(project, tiles_state)).width(Length::FillPortion(1)),
        container(tile_editor(project, tiles_state)).width(Length::FillPortion(14)),
    ]
    .spacing(10)
    .padding(10)
    .into()
}

fn palette_selector<'a>(
    project: &'a Project,
    TilesState {
        palette_name,
        selected_color,
        ..
    }: &'a TilesState,
) -> Element<'a, Message> {
    palette_name
        .as_ref()
        .and_then(|name| project.palette(name))
        .map(|palette| {
            selector(palette.render_colors(), 1, selected_color, |selected| {
                Message::Tile(TileMessage::ColorSelected(selected))
            })
        })
        .unwrap_or_else(|| space().width(Length::Fill).height(Length::Fill).into())
}

fn tile_selector<'a>(
    project: &'a Project,
    TilesState {
        palette_name,
        character_data_name,
        selected_tile,
        ..
    }: &'a TilesState,
) -> Element<'a, Message> {
    palette_name
        .as_ref()
        .zip(character_data_name.as_ref())
        .and_then(|(palette_name, character_data_name)| {
            let palette = project.palette(palette_name)?;
            let character_data = project.character_data(character_data_name)?;
            Some(character_data.render(palette))
        })
        .map(|tiles| {
            selector(tiles, 4, selected_tile, |selected| {
                Message::Tile(TileMessage::TileSelected(selected))
            })
        })
        .unwrap_or_else(|| space().width(Length::Fill).height(Length::Fill).into())
}

fn tile_editor<'a>(
    project: &'a Project,
    TilesState {
        palette_name,
        character_data_name,
        selected_tile,
        selected_pixel,
        ..
    }: &'a TilesState,
) -> Element<'a, Message> {
    palette_name
        .as_ref()
        .zip(character_data_name.as_ref())
        .and_then(|(palette_name, character_data_name)| {
            let palette = project.palette(palette_name)?;
            let character_data = project.character_data(character_data_name)?;
            let tile = character_data.get(*selected_tile)?;
            let message = |selected| Message::Tile(TileMessage::PixelSelected(selected));
            Some(editor(
                tile.render_with(palette),
                *selected_pixel,
                message,
                (8, 8),
            ))
        })
        .unwrap_or_else(|| space().width(Length::Fill).height(Length::Fill).into())
}
