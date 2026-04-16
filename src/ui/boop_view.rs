use iced::advanced::widget::Text;
use crate::project::Project;
use crate::ui::{BoopsState, Message};
use iced::Element;

pub fn boop_view<'a>(
    project: &'a Project,
    boops_state: &'a BoopsState,
) -> Element<'a, Message> {
    Text::from("Boops!").into()
}