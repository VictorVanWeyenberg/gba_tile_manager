use crate::project::Project;
use iced::Element;
use iced::widget::Text;
use iced_aw::{TabLabel, Tabs};

pub struct State {
    project: Project,
    selected_tab: TabId,
}

impl State {
    pub fn new(project: Project) -> Self {
        Self {
            project,
            selected_tab: Default::default()
        }
    }
}

pub enum Message {
    TabSelected(TabId)
}

#[derive(Clone, Default, Eq, PartialEq)]
pub enum TabId {
    #[default]
    Palettes,
    Tiles,
    Screens
}

pub fn view(state: &State) -> Element<'_, Message> {
    Tabs::new(Message::TabSelected)
        .push(TabId::Palettes, TabLabel::Text("Palettes".to_string()), palettes_view(state))
        .push(TabId::Tiles, TabLabel::Text("Tiles".to_string()), tiles_view(state))
        .push(TabId::Screens, TabLabel::Text("Screens".to_string()), screens_view(state))
        .set_active_tab(&state.selected_tab)
        .into()
}

fn palettes_view(state: &State) -> Element<'_, Message> {
    Text::new("Palettes").into()
}

fn tiles_view(state: &State) -> Element<'_, Message> {
    Text::new("Tiles").into()
}

fn screens_view(state: &State) -> Element<'_, Message> {
    Text::new("Screens").into()
}

pub fn update(state: &mut State, message: Message) {
    match message { Message::TabSelected(tab_id) => state.selected_tab = tab_id }
}