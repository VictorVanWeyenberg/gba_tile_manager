use crate::project::Project;
use crate::render::render_cursor;
use crate::ui::{Message, TilesState};
use iced::advanced::image::{FilterMethod, Image};
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Program};
use iced::widget::image::Handle;
use iced::widget::{button, canvas, column, combo_box, responsive, row, scrollable, text_input, Action};
use iced::{mouse, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

const TILE_ROW_N: usize = 4;

pub struct TileSelector<'a, M> {
    tiles: Vec<Handle>,
    message: Box<dyn Fn(usize) -> M>,
    selected_tile: &'a usize,
    origin: Rectangle,
}

impl<'a, M> TileSelector<'a, M> {
    fn dimensions(&self) -> (f32, f32, f32) {
        let width = self.origin.size().width;
        let side = width / TILE_ROW_N as f32;
        let height = side * (self.tiles.len() as f32 / TILE_ROW_N as f32 + 0.5f32);
        (width, height, side)
    }
}

impl<'a, M> Program<M> for TileSelector<'a, M> {
    type State = ();

    fn update(&self, _state: &mut Self::State, event: &Event, bounds: Rectangle, cursor: Cursor) -> Option<Action<M>> {
        if let Event::Mouse(mouse::Event::ButtonPressed(
                                mouse::Button::Left,
                            )) = event
        {
            if let Some(position) = cursor.position_in(bounds) {
                let (width, height, side) = self.dimensions();
                let x = position.x / bounds.width * width / side;
                let y = position.y / bounds.height * height / side;
                let index = TILE_ROW_N * y as usize + x as usize;
                if index >= self.tiles.len() {
                    None
                } else {
                    Some(Action::publish((self.message)(index)))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn draw(
        &self,
        _: &Self::State,
        renderer: &Renderer,
        _: &Theme,
        _: Rectangle,
        _: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let (width, height, side) = self.dimensions();
        let mut frame = Frame::new(renderer, Size::new(side * TILE_ROW_N as f32, height));

        for (n, handle) in self.tiles.iter().enumerate() {
            let x = (n % TILE_ROW_N) * side as usize;
            let y = n / TILE_ROW_N * side as usize;
            frame.draw_image(
                Rectangle::new(Point::new(x as f32, y as f32), Size::new(side, side)),
                Image::new(handle.clone()).filter_method(FilterMethod::Nearest),
            )
        }

        let x = *self.selected_tile % TILE_ROW_N;
        let y = *self.selected_tile / TILE_ROW_N;
        let indicator =
            render_cursor((TILE_ROW_N, ((height / side) + 0.5f32) as usize), x, y).to_handle();
        frame.draw_image(
            Rectangle::new(Point::new(0f32, 0f32), Size::new(width, height)),
            Image::new(indicator).filter_method(FilterMethod::Nearest),
        );

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
                let program = TileSelector {
                    tiles: character_data.render(palette),
                    message: Box::new(Message::TileSelected),
                    selected_tile,
                    origin: Rectangle::new(Point::new(0f32, 0f32), size),
                };
                let (width, height, _) = program.dimensions();
                scrollable(canvas(program).width(width).height(height))
                    .width(Length::Fill)
                    .height(height)
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
    input.spacing(10).padding(10).into()
}
