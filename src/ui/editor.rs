use crate::render::render_cursor;
use iced::advanced::image::FilterMethod;
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Image, Program};
use iced::widget::image::Handle;
use iced::widget::{Action, Canvas};
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

struct Editor<'a, M> {
    handle: Handle,
    location: &'a Point<usize>,
    message: Box<dyn Fn(Point<usize>) -> M>,
    dimensions: (usize, usize),
}

impl<'a, M> Editor<'a, M> {
    pub fn new(
        handle: Handle,
        location: &'a Point<usize>,
        message: impl Fn(Point<usize>) -> M + 'static,
        dimensions: (usize, usize),
    ) -> Self {
        Self {
            handle,
            location,
            message: Box::new(message),
            dimensions,
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
                let x = position.x / bounds.width * self.dimensions.0 as f32;
                let y = position.y / bounds.height * self.dimensions.1 as f32;
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
        let side = bounds.width.min(bounds.height);

        let indicator = render_cursor(self.dimensions, self.location.x, self.location.y).to_handle();

        frame.draw_image(
            Rectangle::new(Point::new(0f32, 0f32), Size::new(side, side)),
            Image::new(self.handle.clone()).filter_method(FilterMethod::Nearest),
        );
        frame.draw_image(
            Rectangle::new(Point::new(0f32, 0f32), Size::new(side, side)),
            Image::new(indicator).filter_method(FilterMethod::Nearest),
        );

        vec![frame.into_geometry()]
    }
}

pub fn editor<'a, M>(
    handle: Handle,
    location: &'a Point<usize>,
    message: impl Fn(Point<usize>) -> M + Copy + 'static,
    dimensions: (usize, usize)
) -> Element<'a, M>
where
    M: 'static,
{
    Canvas::new(Editor::new(handle, location, message, dimensions))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
