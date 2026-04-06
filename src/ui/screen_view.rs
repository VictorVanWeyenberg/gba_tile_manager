use crate::character::Character;
use crate::project::Project;
use crate::ui::editor::editor;
use crate::ui::selector::selector;
use crate::ui::{Message, ScreenMessage, ScreensState, State};
use iced::widget::{button, checkbox, column, combo_box, container, row, space, text_input};
use iced::{Element, Length};

pub fn screen_view<'a>(
    project: &'a Project,
    screens_state: &'a ScreensState,
) -> Element<'a, Message> {
    row![
        column!(
            row![
                text_input("Screen name", &screens_state.new_screen_name)
                    .on_input(|name| Message::Screen(ScreenMessage::NewScreenNameChanged(name))),
                button("Add").on_press(Message::Screen(ScreenMessage::AddScreen))
            ]
            .spacing(10),
            combo_box(
                &screens_state.screen_names,
                "Screen",
                screens_state.selected_screen.as_ref(),
                |name| Message::Screen(ScreenMessage::ScreenSelected(name))
            ),
            combo_box(
                &screens_state.character_map_names,
                "Character map",
                screens_state.selected_character_map.as_ref(),
                |name| Message::Screen(ScreenMessage::CharacterMapSelected(name))
            ),
            combo_box(
                &screens_state.palettes_names,
                "Palette",
                screens_state.selected_palette.as_ref(),
                |name| Message::Screen(ScreenMessage::PaletteSelected(name))
            ),
            row![
                checkbox(screens_state.h_flip)
                    .label("H-Flip")
                    .on_toggle(|h_flip| Message::Screen(ScreenMessage::Flipped {
                        h_flip,
                        v_flip: screens_state.v_flip,
                    })),
                checkbox(screens_state.v_flip)
                    .label("V-Flip")
                    .on_toggle(|v_flip| Message::Screen(ScreenMessage::Flipped {
                        h_flip: screens_state.h_flip,
                        v_flip,
                    })),
            ]
            .spacing(10),
            tile_selector(project, screens_state),
        )
        .spacing(10)
        .width(Length::FillPortion(1)),
        container(screen_editor(project, screens_state)).width(Length::FillPortion(4))
    ]
    .spacing(10)
    .padding(10)
    .into()
}

fn tile_selector<'a>(
    project: &'a Project,
    screens_state: &'a ScreensState,
) -> Element<'a, Message> {
    screens_state
        .selected_palette
        .as_ref()
        .zip(screens_state.selected_character_map.as_ref())
        .and_then(|(palette_name, character_data_name)| {
            let palette = project.palette(palette_name)?;
            let character_data = project.character_data(character_data_name)?;
            Some(selector(
                character_data.render(palette),
                4,
                &screens_state.selected_tile,
                |selected| Message::Screen(ScreenMessage::TileSelected(selected)),
            ))
        })
        .unwrap_or(space().into())
}

fn screen_editor<'a>(
    project: &'a Project,
    screens_state: &'a ScreensState,
) -> Element<'a, Message> {
    screens_state
        .selected_palette
        .as_ref()
        .zip(screens_state.selected_character_map.as_ref())
        .zip(screens_state.selected_screen.as_ref())
        .and_then(|((palette_name, character_data_name), screen_name)| {
            let palette = project.palette(palette_name)?;
            let character_data = project.character_data(character_data_name)?;
            let screen = project.screen_data(screen_name)?;
            Some(editor(
                screen.render(character_data, palette),
                screens_state.selected_character,
                |selected| Message::Screen(ScreenMessage::ScreenClicked(selected)),
                (30, 20),
            ))
        })
        .unwrap_or(space().into())
}

pub fn handle_screen_message(state: &mut State, message: ScreenMessage) {
    let State {
        project,
        screens_state,
        ..
    } = state;
    match message {
        ScreenMessage::NewScreenNameChanged(name) => screens_state.new_screen_name = name,
        ScreenMessage::CharacterMapSelected(name) => {
            screens_state.selected_character_map = Some(name)
        }
        ScreenMessage::PaletteSelected(name) => screens_state.selected_palette = Some(name),
        ScreenMessage::ScreenSelected(selected) => screens_state.selected_screen = Some(selected),
        ScreenMessage::AddScreen => add_screen(project, screens_state),
        ScreenMessage::TileSelected(selected) => tile_selected(project, screens_state, selected),
        ScreenMessage::ScreenClicked(selected) => update_screen(project, screens_state, selected),
        ScreenMessage::Flipped { h_flip, v_flip } => {
            flipped(project, screens_state, h_flip, v_flip)
        }
    }
    if let Some(screen_data) = screens_state
        .selected_screen
        .as_ref()
        .and_then(|name| project.screen_data(name))
    {
        let x = screens_state.selected_character % 30;
        let y = screens_state.selected_character / 30;
        let index = y * 32 + x;
        screens_state.selected_tile = screen_data[index].tile_number();
    }
}

fn tile_selected(project: &mut Project, screens_state: &mut ScreensState, selected: usize) {
    screens_state.selected_tile = selected;
    update_screen(project, screens_state, screens_state.selected_character)
}

fn flipped(project: &mut Project, screens_state: &mut ScreensState, h_flip: bool, v_flip: bool) {
    screens_state.h_flip = h_flip;
    screens_state.v_flip = v_flip;
    update_screen(project, screens_state, screens_state.selected_character)
}

fn update_screen(project: &mut Project, screens_state: &mut ScreensState, selected: usize) {
    let character = Character::new(
        screens_state.selected_tile,
        screens_state.h_flip,
        screens_state.v_flip,
        0,
    );
    let x = selected % 30;
    let y = selected / 30;
    let index = y * 32 + x;
    screens_state
        .selected_screen
        .as_ref()
        .and_then(|name| project.screen_data_mut(name))
        .unwrap()[index] = character;
    screens_state.selected_character = selected;
}

fn add_screen(project: &mut Project, screens_state: &mut ScreensState) {
    project.add_screen_data(&screens_state.new_screen_name);
    screens_state.new_screen_name.clear();
}
