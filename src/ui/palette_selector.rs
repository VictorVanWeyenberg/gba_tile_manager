use crate::palette::Palette;
use crate::project::Project;
use crate::render::render_cursor;
use crate::ui::{Message, TilesState};
use iced::advanced::image::{FilterMethod, Handle, Image};
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Program};
use iced::widget::{Action, canvas, responsive, scrollable};
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme, mouse};

const COLOR_ROW_N: usize = 1;

struct PaletteSelector<'a, M> {
    palette: &'a Palette,
    message: Box<dyn Fn(usize) -> M>,
    selected_color: &'a usize,
    origin: Rectangle,
}

impl<'a, M> PaletteSelector<'a, M> {
    fn dimensions(&self) -> (f32, f32, f32) {
        let width = self.origin.size().width;
        let side = width / COLOR_ROW_N as f32;
        let height = side * 256f32;
        (width, height, side)
    }
}

impl<'a, M> Program<M> for PaletteSelector<'a, M> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<M>> {
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if let Some(position) = cursor.position_in(bounds) {
                let (width, height, side) = self.dimensions();
                let x = position.x / bounds.width * width / side;
                let y = position.y / bounds.height * height / side;
                let index = COLOR_ROW_N * y as usize + x as usize;
                if index >= self.palette.len() {
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
        let mut frame = Frame::new(renderer, Size::new(width, height));

        frame.draw_image(
            Rectangle::new(Point::new(0f32, 0f32), Size::new(width, height)),
            Image::new(self.palette.render_vertical()).filter_method(FilterMethod::Nearest),
        );

        let x = *self.selected_color % COLOR_ROW_N;
        let y = *self.selected_color / COLOR_ROW_N;
        let indicator =
            render_cursor((COLOR_ROW_N, 256), x, y).to_handle();
        frame.draw_image(
            Rectangle::new(Point::new(0f32, 0f32), Size::new(width, height)),
            Image::new(indicator).filter_method(FilterMethod::Nearest),
        );

        vec![frame.into_geometry()]
    }
}

pub fn palette_selector<'a>(
    project: &'a Project,
    TilesState {
        palette_name,
        selected_color,
        ..
    }: &'a TilesState,
) -> Option<Element<'a, Message>> {
    if let Some(palette) = match palette_name {
        Some(palette_name) => project.palette(palette_name),
        _ => return None,
    } {
        Some(
            responsive(|size| {
                let program = PaletteSelector {
                    palette,
                    message: Box::new(Message::TileColorSelected),
                    selected_color,
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
