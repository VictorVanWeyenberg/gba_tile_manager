use crate::project::Project;
use crate::ui::palette_editor::palette_editor;
use iced::widget::Text;
use iced::{Element, Point};
use iced_aw::{TabLabel, Tabs};

mod palette_editor;

pub struct State {
    project: Project,
    selected_tab: TabId,
    palette_state: PaletteState,
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
    cursor: Point<usize>,
}

impl State {
    pub fn new(project: Project) -> Self {
        Self {
            project,
            selected_tab: Default::default(),
            palette_state: Default::default(),
        }
    }
}

#[derive(Clone)]
pub enum Message {
    TabSelected(TabId),
    PaletteClicked(Point<usize>),
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
            tiles_view(state),
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
        cursor,
    }: &'a PaletteState,
) -> Element<'a, Message> {
    let palette = match palette_type {
        PaletteType::Background => project.background_palette(),
        PaletteType::Object => project.object_palette(),
    };
    palette_editor::<'a, '_, Message>(palette, *cursor, &Message::PaletteClicked)
}

fn tiles_view(_: &State) -> Element<'_, Message> {
    Text::new("Tiles").into()
}

fn screens_view(_: &State) -> Element<'_, Message> {
    Text::new("Screens").into()
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::TabSelected(tab_id) => state.selected_tab = tab_id,
        Message::PaletteClicked(point) => state.palette_state.cursor = point,
    }
}
