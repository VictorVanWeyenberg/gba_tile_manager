use iced::advanced::image::{FilterMethod, Handle, Image};
use iced::{mouse, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};
use iced::mouse::Cursor;
use iced::widget::{canvas, responsive, scrollable, Action};
use iced::widget::canvas::{Frame, Geometry, Program};
use crate::project::Project;
use crate::render::render_cursor;
use crate::ui::{Message, TilesState};

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

pub fn tile_selector<'a>(
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