use crate::color::Color;
use crate::ui::Message;
use iced::widget::{container, Text};
use iced::widget::grid;
use iced::{Element, Length};
use iced_aw::number_input;

pub fn palette_input<'a>(selected_color: &Color) -> Element<'a, Message> {
    let Color { r, g, b } = selected_color;
    let rr = *r;
    let gg = *g;
    let bb = *b;
    container(
        grid! {
            Text::new("Red"), number_input(r, 0..32, move |r| {
                Message::PaletteChanged(Color::new(r, gg, bb).unwrap())
            }).ignore_buttons(true),
            Text::new("Green"), number_input(g, 0..32, move |g| {
                Message::PaletteChanged(Color::new(rr, g, bb).unwrap())
            }).ignore_buttons(true),
            Text::new("Blue"), number_input(b, 0..32, move |b| {
                Message::PaletteChanged(Color::new(rr, gg, b).unwrap())
            }).ignore_buttons(true),
        }
        .columns(2)
        .height(Length::Shrink),
    )
    .width(Length::Fixed(200f32))
    .into()
}
