use crate::color::Color;
use crate::ui::{Message, PaletteState};
use iced::widget::{Text, combo_box, container};
use iced::widget::{button, column, grid, row, text_input};
use iced::{Element, Length};
use iced_aw::number_input;

pub fn palette_selector(palette_state: &PaletteState) -> Element<'_, Message> {
    column!(
        row![
            text_input("Add Palette...", &palette_state.new_palette_name)
                .on_input(Message::NewPaletteNameChanged)
                .width(Length::FillPortion(5)),
            button("Add").on_press(Message::AddPalette)
        ],
        combo_box(
            &palette_state.palettes_names,
            "Select Palette...",
            palette_state.palette_name.as_ref(),
            Message::PaletteSelected
        )
    )
    .width(Length::Fixed(200f32))
    .into()
}

pub fn palette_input<'a>(selected_color: Option<&Color>) -> Element<'a, Message> {
    let (r, g, b) = if let Some(Color { r, g, b }) = selected_color {
        (r, g, b)
    } else {
        (&0, &0, &0)
    };
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
