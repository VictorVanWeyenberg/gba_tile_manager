use crate::render::render_cursor;
use iced::advanced::image::{FilterMethod, Handle, Image};
use iced::mouse::{Cursor, ScrollDelta};
use iced::widget::canvas::{Frame, Geometry, Program};
use iced::widget::{canvas, Action};
use iced::{mouse, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};
use std::ops::Add;

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
    type State = f32;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<M>> {
        let (width, height, side) = self.layout(bounds);
        if side - height > *state {
            *state = 0f32;
        }
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if let Some(position) = cursor.position_in(bounds) {
                let x = position.x / width * self.columns as f32;
                let y = (position.y - *state) / height * self.rows() as f32;
                let index = self.columns * y as usize + x as usize;
                if index >= self.images.len() {
                    None
                } else {
                    Some(Action::publish((self.message)(index)))
                }
            } else {
                None
            }
        } else if let Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
            if cursor.position_in(bounds).is_some() {
                *state = (*state)
                    .add(*match delta {
                        ScrollDelta::Lines { y, .. } => y,
                        ScrollDelta::Pixels { y, .. } => y,
                    })
                    .min(0f32)
                    .max(side - height);
                Some(Action::request_redraw())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _: &Theme,
        bounds: Rectangle,
        _: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let (width, height, side) = self.layout(bounds);
        let mut frame = Frame::new(renderer, Size::new(width, height));

        for (n, handle) in self.images.iter().enumerate() {
            let x = (n % self.columns) as f32 * side;
            let y = (n / self.columns) as f32 * side + *state;
            frame.draw_image(
                Rectangle::new(Point::new(x, y), Size::new(side, side)),
                Image::new(handle).filter_method(FilterMethod::Nearest),
            );
        }

        let x = *self.selected % self.columns;
        let y = *self.selected / self.columns;
        let indicator = render_cursor((self.columns, self.rows()), x, y).to_handle();
        frame.draw_image(
            Rectangle::new(Point::new(0f32, *state), Size::new(width, height)),
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
    canvas(Selector {
        images,
        message: Box::new(message),
        selected,
        columns,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
