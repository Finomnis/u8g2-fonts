use core::fmt::Arguments;

use embedded_graphics_core::prelude::{DrawTarget, Point};

use crate::{
    font::SupportsBackgroundColor,
    font_reader::FontReader,
    types::{HorizontalAlignment, RenderedDimensions, VerticalPosition},
    Error, LookupError,
};

use self::content::{ArgsContent, Content};

pub mod content;
mod draw;

pub struct DrawColor<Color> {
    fg: Color,
    bg: Option<Color>,
}

/// A builder for rendering text.
pub struct DrawBuilder<'a, T, Color> {
    content: T,
    position: Point,
    vertical_pos: VerticalPosition,
    horizontal_align: Option<HorizontalAlignment>,
    color: Color,
    font: &'a FontReader,
}

impl<'a> DrawBuilder<'a, ArgsContent<'a>, ()> {
    pub(crate) fn from_args(font: &'a FontReader, args: Arguments<'a>) -> Self {
        Self {
            content: ArgsContent(args),
            position: Point::new(0, 0),
            vertical_pos: VerticalPosition::default(),
            horizontal_align: None,
            color: (),
            font,
        }
    }
}

impl<'a, T, C> DrawBuilder<'a, T, C> {
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

impl<'a, T> DrawBuilder<'a, T, ()> {
    pub fn color<Color>(self, color: Color) -> DrawBuilder<'a, T, DrawColor<Color>> {
        DrawBuilder::<T, DrawColor<Color>> {
            content: self.content,
            position: self.position,
            vertical_pos: self.vertical_pos,
            horizontal_align: self.horizontal_align,
            color: DrawColor {
                fg: color,
                bg: None,
            },
            font: self.font,
        }
    }
}

impl<'a, T, Color> DrawBuilder<'a, T, DrawColor<Color>> {
    pub fn color(mut self, color: Color) -> Self {
        self.color.fg = color;
        self
    }
}

impl<'a, T, Color> DrawBuilder<'a, T, DrawColor<Color>>
where
    T: SupportsBackgroundColor,
{
    pub fn background(mut self, color: Color) -> Result<Self, LookupError> {
        self.color.bg = Some(color);
        Ok(self)
    }
}

impl<'a, T, Color> DrawBuilder<'a, T, DrawColor<Color>>
where
    T: Content,
{
    pub fn draw<Display>(
        &self,
        display: &mut Display,
    ) -> Result<RenderedDimensions, Error<Display::Error>>
    where
        Display: DrawTarget<Color = Color>,
        Display::Error: core::fmt::Debug,
    {
        draw::draw_unaligned(self, display)
    }
}
