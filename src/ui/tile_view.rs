use crate::project::Project;
use crate::ui::editor::editor;
use crate::ui::selector::selector;
use crate::ui::{Message, State, TileMessage, TilesState};
use iced::widget::{button, column, combo_box, container, row, space, text_input};
use iced::{Element, Length};
use crate::tile::Tile;

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

pub fn handle_tile_message(state: &mut State, message: TileMessage) {
    let State {
        project,
        tiles_state,
        ..
    } = state;
    match message {
        TileMessage::CharacterMapSelected(name) => character_map_selected(tiles_state, name),
        TileMessage::PaletteSelected(name) => palette_selected(tiles_state, name),
        TileMessage::TileSelected(selected) => tiles_state.selected_tile = selected,
        TileMessage::ColorSelected(selected) => tiles_state.selected_color = selected,
        TileMessage::PixelSelected(selected) => on_tile_pixel_changed(project, tiles_state, selected),
        TileMessage::AddCharacterMap => add_character_map(project, tiles_state),
        TileMessage::NewCharacterMapNameChanged(name) => tiles_state.new_character_map_name = name,
        TileMessage::AddTile => add_tile(project, tiles_state),
        TileMessage::RemoveTile => remove_tile(project, tiles_state),
        TileMessage::MoveTileUp => move_tile_up(project, tiles_state),
        TileMessage::MoveTileDown => move_tile_down(project, tiles_state),
    }
}

fn move_tile_down(project: &mut Project, tiles_state: &mut TilesState) {
    if let Some(name) = &tiles_state.character_data_name {
        if let Some(character_data) = project.character_data_mut(name) {
            if tiles_state.selected_tile < character_data.len() - 1 {
                character_data
                    .swap(tiles_state.selected_tile, tiles_state.selected_tile + 1);
                tiles_state.selected_tile = tiles_state.selected_tile + 1;
            }
        }
    }
}

fn move_tile_up(project: &mut Project, tiles_state: &mut TilesState) {
    if let Some(name) = &tiles_state.character_data_name {
        if let Some(character_data) = project.character_data_mut(name) {
            if tiles_state.selected_tile > 0 {
                character_data
                    .swap(tiles_state.selected_tile, tiles_state.selected_tile - 1);
                tiles_state.selected_tile = tiles_state.selected_tile - 1;
            }
        }
    }
}

fn remove_tile(project: &mut Project, tiles_state: &mut TilesState) {
    if let Some(name) = &tiles_state.character_data_name {
        if let Some(character_data) = project.character_data_mut(name) {
            character_data.remove(tiles_state.selected_tile);
            if tiles_state.selected_tile >= character_data.len() {
                tiles_state.selected_tile = character_data.len() - 1;
            }
        }
    }
}

fn add_tile(project: &mut Project, tiles_state: &mut TilesState) {
    if let Some(name) = &tiles_state.character_data_name {
        if let Some(character_data) = project.character_data_mut(name) {
            character_data.push(Tile::default())
        }
    }
}

fn add_character_map(project: &mut Project, tiles_state: &mut TilesState) {
    project.add_character_data(&tiles_state.new_character_map_name);
    tiles_state.new_character_map_name.clear();
    tiles_state.character_data_names = combo_box::State::new(project.character_data_names())
}

fn palette_selected(tiles_state: &mut TilesState, name: String) {
    tiles_state.selected_color = 0;
    tiles_state.palette_name = Some(name);
}

fn character_map_selected(tiles_state: &mut TilesState, name: String) {
    tiles_state.selected_tile = 0;
    tiles_state.character_data_name = Some(name);
}

fn on_tile_pixel_changed(project: &mut Project, tiles_state: &mut TilesState, selected: usize) {
    tiles_state.selected_pixel = selected;
    if let Some(character_data_name) = &tiles_state.character_data_name {
        if let Some(character_data) = project.character_data_mut(&character_data_name) {
            character_data[tiles_state.selected_tile][selected] = tiles_state.selected_color as u8
        }
    }
}