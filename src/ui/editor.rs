use crate::palette::Palette;
use crate::render::render_cursor;
use crate::tile::Tile;
use iced::advanced::image::FilterMethod;
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Image, Program};
use iced::widget::image::Handle;
use iced::widget::{responsive, Action, Canvas};
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

struct Editor<'a, M> {
    handle: Handle,
    location: &'a Point<usize>,
    message: Box<dyn Fn(Point<usize>) -> M>,
    origin: Rectangle,
}

impl<'a, M> Editor<'a, M> {
    pub fn new(
        handle: Handle,
        location: &'a Point<usize>,
        message: impl Fn(Point<usize>) -> M + 'static,
        origin: Rectangle,
    ) -> Self {
        Self {
            handle,
            location,
            message: Box::new(message),
            origin
        }
    }
}

impl<'a, M> Program<M> for Editor<'a, M> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<M>> {
        if let Event::Mouse(iced::advanced::mouse::Event::ButtonPressed(
            iced::advanced::mouse::Button::Left,
        )) = event
        {
            if let Some(position) = cursor.position_in(bounds) {
                let x = position.x / bounds.width * 16f32;
                let y = position.y / bounds.height * 16f32;
                return Some(Action::publish((self.message)(Point::new(
                    x as usize, y as usize,
                ))));
            }
        }
        None
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        let indicator = render_cursor((16, 16), self.location.x, self.location.y).to_handle();

        frame.draw_image(
            self.origin,
            Image::new(self.handle.clone()).filter_method(FilterMethod::Nearest),
        );
        frame.draw_image(
            self.origin,
            Image::new(indicator).filter_method(FilterMethod::Nearest),
        );

        vec![frame.into_geometry()]
    }
}

pub fn palette_editor<'a, M>(
    palette: &'a Palette,
    location: &'a Point<usize>,
    message: impl Fn(Point<usize>) -> M + Copy + 'static,
) -> Element<'a, M>
where
    M: 'static,
{

    responsive(move |size| {
        let size = size.width.min(size.height);
        let origin = Rectangle::new(Point::ORIGIN, Size::new(size, size));
        Canvas::new(Editor::new(
            palette.render_square(),
            location,
            message,
            origin,
        ))
            .width(size)
            .height(size)
            .into()
    })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn tile_editor<'a, M>(
    palette: &'a Palette,
    tile: &'a Tile,
    location: &'a Point<usize>,
    message: impl Fn(Point<usize>) -> M + Copy + 'static,
) -> Element<'a, M>
where
    M: 'static,
{

    responsive(move |size| {
        let size = size.width.min(size.height);
        let origin = Rectangle::new(Point::ORIGIN, Size::new(size, size));
        Canvas::new(Editor::new(
            tile.render_with(palette),
            location,
            message,
            origin,
        ))
            .width(size)
            .height(size)
            .into()
    })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
