use crate::project::Project;
use crate::render::{render_cursor, render_palette};
use iced::advanced::image::FilterMethod;
use iced::widget::{Stack, Text, image};
use iced::{Element, Length, Point};
use iced_aw::{TabLabel, Tabs};
use sweeten::mouse_area;

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
    cursor: (usize, usize),
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
    PaletteClicked(Point),
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
    project: &Project,
    PaletteState {
        palette_type,
        cursor: (x, y),
    }: &'a PaletteState,
) -> Element<'a, Message> {
    let palette = match palette_type {
        PaletteType::Background => project.background_palette(),
        PaletteType::Object => project.object_palette(),
    };
    let palette = render_palette(palette).to_handle();
    let background: Element<'_, Message> = image(palette)
        .filter_method(FilterMethod::Nearest)
        .expand(true)
        .into();

    let cursor = render_cursor((16, 16), *x, *y).to_handle();
    let cursor = image(cursor)
        .filter_method(FilterMethod::Nearest)
        .expand(true)
        .into();

    let stack = Stack::with_children(vec![background, cursor])
        .width(Length::Fill)
        .height(Length::Fill);

    mouse_area(stack).on_press(Message::PaletteClicked).into()
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
        Message::PaletteClicked(_) => state.palette_state.cursor = (1, 1),
    }
}
