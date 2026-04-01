use crate::project::Project;
use crate::render::render_cursor;
use crate::ui::{Message, TilesState};
use iced::advanced::image::{FilterMethod, Image};
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Program};
use iced::widget::image::Handle;
use iced::widget::{button, canvas, column, combo_box, responsive, row, scrollable, text_input};
use iced::{Element, Length, Point, Rectangle, Renderer, Size, Theme};

const TILE_ROW_N: usize = 4;

pub struct TileSelector<'a, M> {
    tiles: Vec<Handle>,
    message: Box<dyn Fn(Point<usize>) -> M>,
    selected_tile: &'a usize,
    origin: Rectangle,
}

impl<'a, M> Program<M> for TileSelector<'a, M> {
    type State = ();

    fn draw(
        &self,
        _: &Self::State,
        renderer: &Renderer,
        _: &Theme,
        bounds: Rectangle,
        _: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        let x = *self.selected_tile % TILE_ROW_N;
        let y = *self.selected_tile / TILE_ROW_N;
        let indicator =
            render_cursor((TILE_ROW_N, self.tiles.len() / TILE_ROW_N), x, y).to_handle();
        frame.draw_image(
            self.origin,
            Image::new(indicator).filter_method(FilterMethod::Nearest),
        );

        let side = self.origin.width / TILE_ROW_N as f32;
        for (n, handle) in self.tiles.iter().enumerate() {
            let x = (n % TILE_ROW_N) as f32 * side;
            let y = n as f32 / TILE_ROW_N as f32 * side;
            frame.draw_image(
                Rectangle::new(Point::new(x, y), Size::new(side, side)),
                Image::new(handle.clone()).filter_method(FilterMethod::Nearest),
            )
        }

        vec![frame.into_geometry()]
    }
}

fn tile_selector<'a>(
    project: &'a Project,
    TilesState {
        palette_name,
        character_data_name,
        selected_tile,
        ..
    }: &'a TilesState,
) -> Option<Element<'a, Message>> {
    if let (Some(character_data), Some(palette)) = match (character_data_name, palette_name) {
        (Some(character_data_name), Some(palette_name)) => (
            project.character_data(character_data_name),
            project.palette(palette_name),
        ),
        _ => return None,
    } {
        Some(
            responsive(|size| {
                scrollable(canvas(TileSelector {
                    tiles: character_data.render(palette),
                    message: Box::new(Message::TileClicked),
                    selected_tile,
                    origin: Rectangle::new(Point::new(0f32, 0f32), size),
                }))
                .into()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        )
    } else {
        None
    }
}

pub fn character_map_selector<'a>(
    project: &'a Project,
    tiles_state @ TilesState {
        palette_name,
        character_data_name,
        character_data_names,
        palettes_names,
        new_character_map_name,
        ..
    }: &'a TilesState,
) -> Element<'a, Message> {
    let mut input = column!(
        row![
            text_input("Character Map Name", &new_character_map_name).width(Length::FillPortion(5)),
            button("Add")
                .on_press(Message::AddCharacterMap)
                .width(Length::FillPortion(1))
        ],
        combo_box(
            character_data_names,
            "Pick character map...",
            character_data_name.into(),
            Message::CharacterMapSelected
        ),
        combo_box(
            palettes_names,
            "Pick render palette...",
            palette_name.into(),
            Message::TilesRenderPaletteSelected
        ),
    );
    if let Some(selector) = tile_selector(project, tiles_state) {
        input = input.push(selector);
    }
    input.width(Length::Fixed(200f32)).padding(10).into()
}
