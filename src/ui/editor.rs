use crate::render::render_cursor;
use iced::advanced::image::FilterMethod;
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Image, Program};
use iced::widget::image::Handle;
use iced::widget::{Action, Canvas};
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

struct Editor<M> {
    handle: Handle,
    selected: usize,
    message: Box<dyn Fn(usize) -> M>,
    dimensions: (usize, usize),
}

impl<M> Editor<M> {
    pub fn new(
        handle: Handle,
        selected: usize,
        message: impl Fn(usize) -> M + 'static,
        dimensions: (usize, usize),
    ) -> Self {
        Self {
            handle,
            selected,
            message: Box::new(message),
            dimensions,
        }
    }
}

impl<M> Program<M> for Editor<M> {
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
            let editor_bounds = draw_dimensions(&bounds, &self.dimensions);
            if let Some(position) = cursor.position_in(Rectangle::new(Point::new(bounds.x, bounds.y), editor_bounds)) {
                let x = (position.x / editor_bounds.width * self.dimensions.0 as f32) as usize;
                let y = (position.y / editor_bounds.height * self.dimensions.1 as f32) as usize;
                let idx = y * self.dimensions.0 + x;
                return Some(Action::publish((self.message)(idx)));
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
        let Size { width, height } = draw_dimensions(&bounds, &self.dimensions);

        let x = self.selected % self.dimensions.0;
        let y = self.selected / self.dimensions.0;
        let indicator = render_cursor(self.dimensions, x, y).to_handle();

        frame.draw_image(
            Rectangle::new(Point::new(0f32, 0f32), Size::new(width, height)),
            Image::new(self.handle.clone()).filter_method(FilterMethod::Nearest),
        );
        frame.draw_image(
            Rectangle::new(Point::new(0f32, 0f32), Size::new(width, height)),
            Image::new(indicator).filter_method(FilterMethod::Nearest),
        );

        vec![frame.into_geometry()]
    }
}

fn draw_dimensions(bounds: &Rectangle, dimensions: &(usize, usize)) -> Size {
    let scale = (bounds.width / dimensions.0 as f32)
        .min(bounds.height / dimensions.1 as f32);
    Size::new(dimensions.0 as f32 * scale, dimensions.1 as f32 * scale)
}

pub fn editor<'a, M>(
    handle: Handle,
    selected: usize,
    message: impl Fn(usize) -> M + Copy + 'static,
    dimensions: (usize, usize),
) -> Element<'a, M>
where
    M: 'static,
{
    Canvas::new(Editor::new(handle, selected, message, dimensions))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
