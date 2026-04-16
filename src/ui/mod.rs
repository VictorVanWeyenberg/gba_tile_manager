use crate::color::Color;
use crate::project::Project;
use crate::ui::boop_view::boop_view;
use crate::ui::palette_view::palette_view;
use crate::ui::screen_view::screen_view;
use crate::ui::tile_view::tile_view;
use iced::widget::combo_box;
use iced::Element;
use iced_aw::{TabLabel, Tabs};

mod boop_view;
mod editor;
mod palette_view;
mod screen_view;
mod selector;
mod tile_view;

pub struct State {
    project: Project,
    selected_tab: TabId,
    palette_state: PaletteState,
    tiles_state: TilesState,
    screens_state: ScreensState,
    boops_state: BoopsState,
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

pub struct ScreensState {
    new_screen_name: String,
    selected_tile: usize,
    selected_character: usize,
    screen_names: combo_box::State<String>,
    character_map_names: combo_box::State<String>,
    palettes_names: combo_box::State<String>,
    selected_screen: Option<String>,
    selected_character_map: Option<String>,
    selected_palette: Option<String>,
    h_flip: bool,
    v_flip: bool,
}

impl ScreensState {
    fn new(project: &Project) -> Self {
        Self {
            new_screen_name: "".to_string(),
            selected_tile: 0,
            selected_character: 0,
            screen_names: combo_box::State::new(project.screen_names()),
            character_map_names: combo_box::State::new(project.character_data_names()),
            palettes_names: combo_box::State::new(project.palette_names()),
            selected_screen: None,
            selected_character_map: None,
            selected_palette: None,
            h_flip: false,
            v_flip: false,
        }
    }
}

pub struct BoopsState {
    new_boops_name: String,
    boops_names: combo_box::State<String>,
    selected_boops_name: Option<String>,
    screen_names: combo_box::State<String>,
    selected_screen: Option<String>,
    character_map_names: combo_box::State<String>,
    selected_character_map: Option<String>,
    palette_names: combo_box::State<String>,
    selected_palette: Option<String>,
}

impl BoopsState {
    fn new(project: &Project) -> Self {
        Self {
            new_boops_name: "".to_string(),
            boops_names: combo_box::State::new(project.boop_names()),
            selected_boops_name: None,
            screen_names: combo_box::State::new(project.screen_names()),
            selected_screen: None,
            character_map_names: combo_box::State::new(project.character_data_names()),
            selected_character_map: None,
            palette_names: combo_box::State::new(project.palette_names()),
            selected_palette: None,
        }
    }
}

impl State {
    pub fn new(project: Project) -> Self {
        let palette_state = PaletteState::new(&project);
        let tiles_state = TilesState::new(&project);
        let screens_state = ScreensState::new(&project);
        let boops_state = BoopsState::new(&project);
        Self {
            project,
            selected_tab: Default::default(),
            palette_state,
            tiles_state,
            screens_state,
            boops_state,
        }
    }
}

#[derive(Clone)]
pub enum Message {
    TabSelected(TabId),
    Palette(PaletteMessage),
    Tile(TileMessage),
    Screen(ScreenMessage),
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

#[derive(Clone)]
pub enum ScreenMessage {
    NewScreenNameChanged(String),
    CharacterMapSelected(String),
    PaletteSelected(String),
    ScreenSelected(String),
    TileSelected(usize),
    ScreenClicked(usize),
    Flipped {
        h_flip: bool,
        v_flip: bool,
    },
    AddScreen,
}

#[derive(Clone, Default, Eq, PartialEq)]
pub enum TabId {
    #[default]
    Palettes,
    Tiles,
    Screens,
    Boops,
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
            screens_view(&state.project, &state.screens_state),
        )
        .push(
            TabId::Boops,
            TabLabel::Text("Boops".to_string()),
            boops_view(&state.project, &state.boops_state),
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
    tile_view(project, tiles_state)
}

fn screens_view<'a>(project: &'a Project, screens_state: &'a ScreensState) -> Element<'a, Message> {
    screen_view(project, screens_state)
}

fn boops_view<'a>(project: &'a Project, boops_state: &'a BoopsState) -> Element<'a, Message> {
    boop_view(project, boops_state)
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::TabSelected(tab_id) => tab_selected(state, tab_id),
        Message::Palette(message) => palette_view::handle_palette_message(state, message),
        Message::Tile(message) => tile_view::handle_tile_message(state, message),
        Message::Screen(message) => screen_view::handle_screen_message(state, message),
    }
}

fn tab_selected(state: &mut State, tab_id: TabId) {
    match tab_id {
        TabId::Tiles => {
            state.tiles_state.palettes_names = combo_box::State::new(state.project.palette_names())
        }
        TabId::Screens => {
            state.screens_state.palettes_names = combo_box::State::new(state.project.palette_names());
            state.screens_state.character_map_names = combo_box::State::new(state.project.character_data_names())
        }
        TabId::Boops => {
            state.boops_state.palette_names = combo_box::State::new(state.project.palette_names());
            state.boops_state.screen_names = combo_box::State::new(state.project.screen_names());
        }
        _ => {}
    };
    state.selected_tab = tab_id
}
