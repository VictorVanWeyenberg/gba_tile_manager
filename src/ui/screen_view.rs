use iced::Element;
use iced::widget::space;
use crate::project::Project;
use crate::ui::{Message, ScreenMessage, ScreensState, State};

pub fn screen_view<'a>(project: &'a Project, screens_state: &'a ScreensState) -> Element<'a, Message> {
    space().into()
}

pub fn handle_screen_message(state: &mut State, message: ScreenMessage) {
    match message {
        ScreenMessage::NewScreenNameChanged(_) => {}
        ScreenMessage::CharacterMapSelected(_) => {}
        ScreenMessage::PaletteSelected(_) => {}
        ScreenMessage::ScreenSelected(_) => {}
        ScreenMessage::AddScreen => {}
        ScreenMessage::RemoveScreen => {}
    }
}