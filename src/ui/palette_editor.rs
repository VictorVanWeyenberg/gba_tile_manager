use crate::palette::Palette;
use crate::render::{render_cursor, render_palette};
use iced::advanced::image::FilterMethod;
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Image, Program};
use iced::widget::{Action, Canvas, responsive};
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

struct PaletteEditor<'a, 'm, Message> {
    palette: &'a Palette,
    location: Point<usize>,
    message: &'m dyn Fn(Point<usize>) -> Message,
    origin: Rectangle,
}

impl<'a, 'm, Message> PaletteEditor<'a, 'm, Message> {
    pub fn new(
        palette: &'a Palette,
        location: Point<usize>,
        message: &'m impl Fn(Point<usize>) -> Message,
        origin: Rectangle,
    ) -> Self {
        Self {
            palette,
            location,
            message,
            origin,
        }
    }
}

impl<'a, 'm, Message> Program<Message> for PaletteEditor<'a, 'm, Message> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<Message>> {
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

        let palette = render_palette(self.palette).to_handle();
        let indicator = render_cursor((16, 16), self.location.x, self.location.y).to_handle();

        frame.draw_image(
            self.origin,
            Image::new(palette).filter_method(FilterMethod::Nearest),
        );
        frame.draw_image(
            self.origin,
            Image::new(indicator).filter_method(FilterMethod::Nearest),
        );

        vec![frame.into_geometry()]
    }
}

pub fn palette_editor<'a, 'm, Message: 'm>(
    palette: &'a Palette,
    location: Point<usize>,
    message: &'m impl Fn(Point<usize>) -> Message,
) -> Element<'a, Message>
where
    'm: 'a,
{
    responsive(move |size| {
        let side = size.width.min(size.height);
        let origin = Rectangle::new(Point::ORIGIN, Size::new(side, side));
        Canvas::new(PaletteEditor::new(palette, location, message, origin))
            .width(side)
            .height(side)
            .into()
    })
    .into()
}
