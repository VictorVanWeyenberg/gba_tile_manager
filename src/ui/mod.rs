use crate::color::Color;
use crate::project::Project;
use crate::tile::Tile;
use crate::ui::palette_view::palette_view;
use crate::ui::tile_view::character_map_selector;
use iced::Element;
use iced::widget::{Text, combo_box};
use iced_aw::{TabLabel, Tabs};

mod editor;
mod palette_view;
mod selector;
mod tile_view;

pub struct State {
    project: Project,
    selected_tab: TabId,
    palette_state: PaletteState,
    tiles_state: TilesState,
}

pub struct PaletteState {
    palette_name: Option<String>,
    new_palette_name: String,
    palettes_names: combo_box::State<String>,
    selected_color: usize,
}

impl PaletteState {
    fn new(project: &Project) -> Self {
        Self {
            palette_name: None,
            new_palette_name: "".to_string(),
            palettes_names: combo_box::State::new(project.palette_names()),
            selected_color: Default::default(),
        }
    }
}

pub struct TilesState {
    palette_name: Option<String>,
    character_data_name: Option<String>,
    selected_tile: usize,
    character_data_names: combo_box::State<String>,
    palettes_names: combo_box::State<String>,
    new_character_map_name: String,
    selected_color: usize,
    selected_pixel: usize,
}

impl TilesState {
    fn new(project: &Project) -> Self {
        Self {
            palette_name: None,
            character_data_name: None,
            selected_tile: 0,
            character_data_names: combo_box::State::new(project.character_data_names()),
            palettes_names: combo_box::State::new(project.palette_names()),
            new_character_map_name: "".to_string(),
            selected_color: 0,
            selected_pixel: 0,
        }
    }
}

impl State {
    pub fn new(project: Project) -> Self {
        let palette_state = PaletteState::new(&project);
        let tiles_state = TilesState::new(&project);
        Self {
            project,
            selected_tab: Default::default(),
            palette_state,
            tiles_state,
        }
    }
}

#[derive(Clone)]
pub enum Message {
    TabSelected(TabId),
    Palette(PaletteMessage),
    Tile(TileMessage),
}

#[derive(Clone)]
pub enum PaletteMessage {
    NewPaletteNameChanged(String),
    PaletteSelected(String),
    PaletteClicked(usize),
    PaletteChanged(Color),
    AddPalette,
}

#[derive(Clone)]
pub enum TileMessage {
    NewCharacterMapNameChanged(String),
    CharacterMapSelected(String),
    PaletteSelected(String),
    TileSelected(usize),
    ColorSelected(usize),
    PixelSelected(usize),
    AddCharacterMap,
    AddTile,
    RemoveTile,
    MoveTileUp,
    MoveTileDown,
}

#[derive(Clone, Default, Eq, PartialEq)]
pub enum TabId {
    #[default]
    Palettes,
    Tiles,
    Screens,
}

pub fn view(state: &State) -> Element<'_, Message> {
    Tabs::new(Message::TabSelected)
        .push(
            TabId::Palettes,
            TabLabel::Text("Palettes".to_string()),
            palettes_view(&state.project, &state.palette_state),
        )
        .push(
            TabId::Tiles,
            TabLabel::Text("Tiles".to_string()),
            tiles_view(&state.project, &state.tiles_state),
        )
        .push(
            TabId::Screens,
            TabLabel::Text("Screens".to_string()),
            screens_view(state),
        )
        .set_active_tab(&state.selected_tab)
        .into()
}

fn palettes_view<'a>(
    project: &'a Project,
    palette_state: &'a PaletteState,
) -> Element<'a, Message> {
    palette_view(project, palette_state)
}

fn tiles_view<'a>(project: &'a Project, tiles_state: &'a TilesState) -> Element<'a, Message> {
    character_map_selector(project, tiles_state)
}

fn screens_view(_: &State) -> Element<'_, Message> {
    Text::new("Screens").into()
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::TabSelected(tab_id) => state.selected_tab = tab_id,
        Message::Palette(message) => handle_palette_message(state, message),
        Message::Tile(message) => handle_tile_message(state, message),
    }
}

fn handle_palette_message(state: &mut State, message: PaletteMessage) {
    let State {
        project,
        palette_state,
        ..
    } = state;
    match message {
        PaletteMessage::NewPaletteNameChanged(name) => palette_state.new_palette_name = name,
        PaletteMessage::PaletteClicked(selected) => palette_state.selected_color = selected,
        PaletteMessage::PaletteChanged(color) => on_palette_changed(project, palette_state, color),
        PaletteMessage::AddPalette => {
            project.add_palette(&palette_state.new_palette_name);
            palette_state.new_palette_name.clear();
            palette_state.palettes_names = combo_box::State::new(state.project.palette_names());
            state.tiles_state.palettes_names = combo_box::State::new(state.project.palette_names())
        }
        PaletteMessage::PaletteSelected(name) => palette_state.palette_name = Some(name),
    }
}

fn handle_tile_message(state: &mut State, message: TileMessage) {
    let State {
        project,
        tiles_state,
        ..
    } = state;
    match message {
        TileMessage::CharacterMapSelected(name) => {
            tiles_state.selected_tile = 0;
            tiles_state.character_data_name = Some(name);
        }
        TileMessage::PaletteSelected(name) => {
            tiles_state.selected_color = 0;
            tiles_state.palette_name = Some(name);
        }
        TileMessage::TileSelected(selected) => tiles_state.selected_tile = selected,
        TileMessage::ColorSelected(selected) => tiles_state.selected_color = selected,
        TileMessage::PixelSelected(selected) => {
            on_tile_pixel_changed(project, tiles_state, selected)
        }
        TileMessage::AddCharacterMap => {
            project.add_character_data(&tiles_state.new_character_map_name);
            tiles_state.new_character_map_name.clear();
            tiles_state.character_data_names = combo_box::State::new(project.character_data_names())
        }
        TileMessage::NewCharacterMapNameChanged(name) => {
            tiles_state.new_character_map_name = name;
        }
        TileMessage::AddTile => {
            if let Some(name) = &tiles_state.character_data_name {
                if let Some(character_data) = project.character_data_mut(name) {
                    character_data.push(Tile::default())
                }
            }
        }
        TileMessage::RemoveTile => {
            if let Some(name) = &tiles_state.character_data_name {
                if let Some(character_data) = project.character_data_mut(name) {
                    character_data.remove(tiles_state.selected_tile);
                    if tiles_state.selected_tile >= character_data.len() {
                        tiles_state.selected_tile = character_data.len() - 1;
                    }
                }
            }
        }
        TileMessage::MoveTileUp => {
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
        TileMessage::MoveTileDown => {
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
    }
}

fn on_palette_changed(project: &mut Project, palette_state: &mut PaletteState, color: Color) {
    if let Some(palette_name) = &palette_state.palette_name {
        let palette = project.palette_mut(&palette_name).unwrap();
        palette.set_color(*(&palette_state.selected_color), color)
    }
}

fn on_tile_pixel_changed(project: &mut Project, tiles_state: &mut TilesState, selected: usize) {
    tiles_state.selected_pixel = selected;
    if let Some(character_data_name) = &tiles_state.character_data_name {
        if let Some(character_data) = project.character_data_mut(&character_data_name) {
            character_data[tiles_state.selected_tile][selected] = tiles_state.selected_color as u8
        }
    }
}
