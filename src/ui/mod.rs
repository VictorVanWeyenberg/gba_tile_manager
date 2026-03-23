use crate::color::Color;
use crate::palette::Palette;
use crate::project::Project;
use crate::ui::editor::{palette_editor, tile_editor};
use crate::ui::palette_input::palette_input;
use iced::widget::{Text, row};
use iced::{Element, Point};
use iced_aw::{TabLabel, Tabs};

mod editor;
mod palette_input;

pub struct State {
    project: Project,
    selected_tab: TabId,
    palette_state: PaletteState,
    tiles_state: TilesState,
}

#[derive(Default)]
pub enum PaletteType {
    #[default]
    Background,
    Object,
}

#[derive(Default)]
pub struct PaletteState {
    palette_type: PaletteType,
    location: Point<usize>,
}

pub struct TilesState {
    palette_type: PaletteType,
    location: Point<usize>,
    character_map: String,
    tile_index: usize,
}

impl Default for TilesState {
    fn default() -> Self {
        Self {
            palette_type: Default::default(),
            location: Default::default(),
            character_map: "empty_art".to_string(),
            tile_index: 23,
        }
    }
}

impl State {
    pub fn new(project: Project) -> Self {
        Self {
            project,
            selected_tab: Default::default(),
            palette_state: Default::default(),
            tiles_state: Default::default(),
        }
    }
}

#[derive(Clone)]
pub enum Message {
    TabSelected(TabId),
    PaletteClicked(Point<usize>),
    PaletteChanged(Color),
    TileClicked(Point<usize>),
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
    PaletteState {
        palette_type,
        location: cursor,
    }: &'a PaletteState,
) -> Element<'a, Message> {
    let palette = get_selected_palette(project, palette_type);
    let selected_color = get_palette_color_at_point(palette, cursor);
    row! {
        palette_input(selected_color),
        palette_editor(palette, *cursor, Message::PaletteClicked)
    }
    .spacing(10)
    .padding(10)
    .into()
}

fn get_selected_palette<'p>(project: &'p Project, palette_type: &PaletteType) -> &'p Palette {
    match palette_type {
        PaletteType::Background => project.background_palette(),
        PaletteType::Object => project.object_palette(),
    }
}

fn tiles_view<'a>(
    project: &'a Project,
    TilesState {
        palette_type,
        location: cursor,
        character_map,
        tile_index,
    }: &'a TilesState,
) -> Element<'a, Message> {
    let tile = project
        .screens()
        .get(character_map)
        .unwrap()
        .bg1_character_data
        .get(*tile_index)
        .unwrap();
    let palette = get_selected_palette(project, palette_type);
    tile_editor(palette, tile, *cursor, Message::TileClicked)
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
        Message::TileClicked(point) => state.tiles_state.location = point,
    }
}

fn get_palette_color_at_point<'a>(palette: &'a Palette, point: &Point<usize>) -> &'a Color {
    palette.get(point.y * 16 + point.x).unwrap_or(&palette[0])
}

fn on_palette_changed(state: &mut State, color: Color) {
    let palette_type = &state.palette_state.palette_type;
    let project = &mut state.project;
    let point = &state.palette_state.location;
    let palette = match palette_type {
        PaletteType::Background => project.background_palette_mut(),
        PaletteType::Object => project.object_palette_mut(),
    };
    set_palette_color_at_point(palette, point, color)
}

fn set_palette_color_at_point(palette: &mut Palette, point: &Point<usize>, color: Color) {
    let index = point.y * 16 + point.x;
    palette.set_color(index, color)
}
