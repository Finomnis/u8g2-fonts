use core::fmt::Arguments;

use embedded_graphics_core::prelude::{DrawTarget, Point};

use crate::{
    types::{HorizontalAlignment, RenderedDimensions, VerticalPosition},
    Error, LookupError,
};

use self::content::{ArgsContent, Content};

pub mod content;

pub struct DrawColor<Color> {
    fg: Color,
    bg: Option<Color>,
}

/// A builder for rendering text.
pub struct DrawBuilder<T, Color> {
    content: T,
    position: Point,
    vertical_pos: VerticalPosition,
    horizontal_align: Option<HorizontalAlignment>,
    color: Color,
}

impl<'a> DrawBuilder<ArgsContent<'a>, ()> {
    pub(crate) fn from_args(args: Arguments<'a>) -> Self {
        Self {
            content: ArgsContent(args),
            position: Point::new(0, 0),
            vertical_pos: VerticalPosition::default(),
            horizontal_align: None,
            color: (),
        }
    }
}

impl<T, C> DrawBuilder<T, C> {
    pub fn position(mut self, position: Point, vertical_pos: VerticalPosition) -> Self {
        self.position = position;
        self.vertical_pos = vertical_pos;
        self
    }

    pub fn alignment(mut self, horizontal_align: HorizontalAlignment) -> Self {
        self.horizontal_align = Some(horizontal_align);
        self
    }

    pub fn compute_dimensions(&self) -> Result<RenderedDimensions, LookupError> {
        todo!()
    }
}

impl<T> DrawBuilder<T, ()> {
    pub fn color<Color>(self, color: Color) -> DrawBuilder<T, DrawColor<Color>> {
        DrawBuilder::<T, DrawColor<Color>> {
            content: self.content,
            position: self.position,
            vertical_pos: self.vertical_pos,
            horizontal_align: self.horizontal_align,
            color: DrawColor {
                fg: color,
                bg: None,
            },
        }
    }
}

impl<T, Color> DrawBuilder<T, DrawColor<Color>> {
    pub fn color(mut self, color: Color) -> Self {
        self.color.fg = color;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.color.bg = Some(color);
        self
    }

    pub fn draw<Display>(
        &self,
        display: &mut Display,
    ) -> Result<RenderedDimensions, Error<Display::Error>>
    where
        Display: DrawTarget<Color = Color>,
        Display::Error: core::fmt::Debug,
    {
        todo!()
    }
}
