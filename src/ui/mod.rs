use crate::color::Color;
use crate::palette::Palette;
use crate::project::Project;
use crate::ui::editor::palette_editor;
use crate::ui::palette_input::{palette_input, palette_selector};
use iced::widget::{Text, column, combo_box, row};
use iced::{Element, Point};
use iced_aw::{TabLabel, Tabs};
use crate::ui::tile_view::character_map_selector;

mod editor;
mod palette_input;
mod tile_selector;
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
    location: Point<usize>,
}

impl PaletteState {
    fn new(project: &Project) -> Self {
        Self {
            palette_name: None,
            new_palette_name: "".to_string(),
            palettes_names: combo_box::State::new(project.palette_names()),
            location: Default::default(),
        }
    }
}

pub struct TilesState {
    palette_name: Option<String>,
    character_data_name: Option<String>,
    selected_tile: usize,
    character_data_names: combo_box::State<String>,
    palettes_names: combo_box::State<String>,
    location: Point<usize>,
    new_character_map_name: String,
}

impl TilesState {
    fn new(project: &Project) -> Self {
        Self {
            palette_name: None,
            character_data_name: None,
            selected_tile: 0,
            character_data_names: combo_box::State::new(project.character_data_names()),
            palettes_names: combo_box::State::new(project.palette_names()),
            location: Default::default(),
            new_character_map_name: "".to_string(),
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
    NewPaletteNameChanged(String),
    PaletteClicked(Point<usize>),
    PaletteChanged(Color),
    TileSelected(usize),
    PaletteSelected(String),
    AddPalette,
    AddCharacterMap,
    CharacterMapSelected(String),
    TilesRenderPaletteSelected(String),
}

#[derive(Clone, Default, Eq, PartialEq)]
pub enum TabId {
    #[default]
    Palettes,
    Tiles,
    Screens,
}

pub fn view(state: &State) -> Element<Message> {
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
    let PaletteState {
        palette_name,
        location,
        ..
    } = palette_state;
    let selector = palette_selector(palette_state);
    match palette_name {
        None => row![selector,],
        Some(palette_name) => {
            let palette = project.palette(palette_name).unwrap();
            let selected_color = palette.get(location.y * 16 + location.x);
            row! {
                column!(
                    selector,
                    palette_input(selected_color)
                ),
                palette_editor(palette, location, Message::PaletteClicked)
            }
        }
    }
    .spacing(10)
    .padding(10)
    .into()
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
        Message::PaletteClicked(point) => {
            state.palette_state.location = point;
        }
        Message::PaletteChanged(color) => on_palette_changed(state, color),
        Message::TileSelected(selected) => state.tiles_state.selected_tile = selected,
        Message::NewPaletteNameChanged(name) => state.palette_state.new_palette_name = name,
        Message::PaletteSelected(name) => state.palette_state.palette_name = Some(name),
        Message::AddPalette => {
            state
                .project
                .add_palette(&state.palette_state.new_palette_name);
            state.palette_state.new_palette_name.clear();
            state.palette_state.palettes_names =
                combo_box::State::new(state.project.palette_names());
            state.tiles_state.palettes_names = combo_box::State::new(state.project.palette_names())
        }
        Message::AddCharacterMap => {
            state
                .project
                .add_character_data(&state.tiles_state.new_character_map_name);
            state.tiles_state.new_character_map_name.clear();
            state.tiles_state.character_data_names =
                combo_box::State::new(state.project.character_data_names())
        }
        Message::CharacterMapSelected(name) => state.tiles_state.character_data_name = Some(name),
        Message::TilesRenderPaletteSelected(name) => state.tiles_state.palette_name = Some(name),
    }
}

fn on_palette_changed(state: &mut State, color: Color) {
    if let Some(palette_name) = &state.palette_state.palette_name {
        let project = &mut state.project;
        let point = &state.palette_state.location;
        let palette = project.palette_mut(&palette_name).unwrap();
        set_palette_color_at_point(palette, point, color)
    }
}

fn set_palette_color_at_point(palette: &mut Palette, point: &Point<usize>, color: Color) {
    let index = point.y * 16 + point.x;
    palette.set_color(index, color)
}
