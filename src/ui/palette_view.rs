use crate::color::Color;
use crate::project::Project;
use crate::ui::editor::editor;
use crate::ui::{Message, PaletteMessage, PaletteState, State};
use iced::widget::{Text, combo_box, container};
use iced::widget::{button, column, grid, row, text_input};
use iced::{Element, Length};
use iced_aw::number_input;

fn palette_selector(palette_state: &PaletteState) -> Element<'_, Message> {
    column!(
        row![
            text_input("Add Palette...", &palette_state.new_palette_name)
                .on_input(|name| Message::Palette(PaletteMessage::NewPaletteNameChanged(name)))
                .width(Length::FillPortion(5)),
            button("Add").on_press(Message::Palette(PaletteMessage::AddPalette))
        ]
        .spacing(10),
        combo_box(
            &palette_state.palettes_names,
            "Select Palette...",
            palette_state.palette_name.as_ref(),
            |name| Message::Palette(PaletteMessage::PaletteSelected(name))
        )
    )
    .spacing(10)
    .into()
}

fn palette_input<'a>(selected_color: Option<&Color>) -> Element<'a, Message> {
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
                Message::Palette(PaletteMessage::PaletteChanged(Color::new(r, gg, bb).unwrap()))
            }).ignore_buttons(true),
            Text::new("Green"), number_input(g, 0..32, move |g| {
                Message::Palette(PaletteMessage::PaletteChanged(Color::new(rr, g, bb).unwrap()))
            }).ignore_buttons(true),
            Text::new("Blue"), number_input(b, 0..32, move |b| {
                Message::Palette(PaletteMessage::PaletteChanged(Color::new(rr, gg, b).unwrap()))
            }).ignore_buttons(true),
        }
        .columns(2)
        .height(Length::Shrink)
        .spacing(10),
    )
    .into()
}

pub fn palette_view<'a>(
    project: &'a Project,
    palette_state: &'a PaletteState,
) -> Element<'a, Message> {
    let PaletteState {
        palette_name,
        selected_color,
        ..
    } = palette_state;
    let selector = palette_selector(palette_state);
    match palette_name {
        None => row![selector,].spacing(10),
        Some(palette_name) => {
            let palette = project.palette(palette_name).unwrap();
            let color = palette.get(*selected_color);
            let editor = editor(
                palette.render_square(),
                *selected_color,
                |selected| Message::Palette(PaletteMessage::PaletteClicked(selected)),
                (16, 16),
            );
            row! {
                column!(
                    selector,
                    palette_input(color)
                ).spacing(10)
                .width(Length::FillPortion(1)),
                container(editor).width(Length::FillPortion(5))
            }
            .spacing(10)
        }
    }
    .spacing(10)
    .padding(10)
    .into()
}

pub fn handle_palette_message(state: &mut State, message: PaletteMessage) {
    let State {
        project,
        palette_state,
        ..
    } = state;
    match message {
        PaletteMessage::NewPaletteNameChanged(name) => palette_state.new_palette_name = name,
        PaletteMessage::PaletteClicked(selected) => palette_state.selected_color = selected,
        PaletteMessage::PaletteChanged(color) => on_palette_changed(project, palette_state, color),
        PaletteMessage::AddPalette => add_palette(palette_state, project),
        PaletteMessage::PaletteSelected(name) => palette_state.palette_name = Some(name),
    }
}

fn add_palette(palette_state: &mut PaletteState, project: &mut Project) {
    project.add_palette(&palette_state.new_palette_name);
    palette_state.new_palette_name.clear();
    palette_state.palettes_names = combo_box::State::new(project.palette_names());
}

fn on_palette_changed(project: &mut Project, palette_state: &mut PaletteState, color: Color) {
    if let Some(palette_name) = &palette_state.palette_name {
        let palette = project.palette_mut(&palette_name).unwrap();
        palette.set_color(*(&palette_state.selected_color), color)
    }
}
