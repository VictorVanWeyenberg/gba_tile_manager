use crate::render::{render_cursor, ToHandle};
use iced::advanced::image::{FilterMethod, Handle, Image};
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Program};
use iced::widget::{canvas, scrollable, Action};
use iced::{mouse, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

struct Selector<'a, M> {
    images: Vec<Handle>,
    message: Box<dyn Fn(usize) -> M>,
    selected: &'a usize,
    columns: usize,
}

impl<'a, M> Selector<'a, M> {
    fn rows(&self) -> usize {
        (self.images.len() + self.columns - 1) / self.columns
    }

    /// Returns `(width, height, side)` — the total canvas size and the cell size.
    fn layout(&self, bounds: Rectangle) -> (f32, f32, f32) {
        let width = bounds.size().width;
        let side = width / self.columns as f32;
        let height = side * self.rows() as f32;
        (width, height, side)
    }
}

impl<'a, M> Program<M> for Selector<'a, M> {
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
                let (width, height, side) = self.layout(bounds);
                let x = position.x / bounds.width * width / side;
                let y = position.y / bounds.height * height / side;
                let index = self.columns * y as usize + x as usize;
                if index >= self.images.len() {
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
        bounds: Rectangle,
        _: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let (width, height, side) = self.layout(bounds);
        let cols = self.columns;
        let mut frame = Frame::new(renderer, Size::new(side * cols as f32, height));

        for (n, handle) in self.images.iter().enumerate() {
            let x = (n % cols) as f32 * side;
            let y = (n / cols) as f32 * side;
            frame.draw_image(
                Rectangle::new(Point::new(x, y), Size::new(side, side)),
                Image::new(handle).filter_method(FilterMethod::Nearest),
            );
        }

        let x = *self.selected % cols;
        let y = *self.selected / cols;
        let indicator = render_cursor((cols, self.rows()), x, y).to_handle();
        frame.draw_image(
            Rectangle::new(Point::new(0f32, 0f32), Size::new(width, height)),
            Image::new(indicator).filter_method(FilterMethod::Nearest),
        );

        vec![frame.into_geometry()]
    }
}

/// A generic scrollable grid selector.
///
/// # Arguments
/// - `images`   — the rendered image handles to display in the grid
/// - `columns`  — number of items per row; rows are derived from this and the image count
/// - `selected` — index of the currently selected item
/// - `message`  — called with the clicked index to produce a message
pub fn selector<'a, M: 'a>(
    images: Vec<Handle>,
    columns: usize,
    selected: &'a usize,
    message: impl Fn(usize) -> M + 'static,
) -> Element<'a, M> {
    let program = Selector {
        images,
        message: Box::new(message),
        selected,
        columns,
    };
    scrollable(canvas(program)).width(Length::Fill).into()
}
