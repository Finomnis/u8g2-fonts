use embedded_graphics_core::{
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
};

use crate::{
    font_reader::FontReader,
    types::{HorizontalAlignment, RenderedDimensions, VerticalPosition},
    Error, LookupError,
};

use self::content::Content;

pub mod content;

mod compute_dimensions;
mod draw;

pub struct DrawColor<Color> {
    fg: Color,
    bg: Option<Color>,
}

/// A builder for rendering text.
pub struct DrawBuilder<'a, T, Color, Align> {
    content: T,
    position: Point,
    vertical_pos: VerticalPosition,
    font: &'a FontReader,
    color: Color,
    horizontal_align: Align,
}

impl<'a, T> DrawBuilder<'a, T, (), ()>
where
    T: Content,
{
    pub(crate) fn new(font: &'a FontReader, content: T) -> Self {
        Self {
            content,
            position: Point::new(0, 0),
            vertical_pos: VerticalPosition::default(),
            horizontal_align: (),
            color: (),
            font,
        }
    }
}

impl<'a, T, C, A> DrawBuilder<'a, T, C, A> {
    pub fn position(mut self, position: Point, vertical_pos: VerticalPosition) -> Self {
        self.position = position;
        self.vertical_pos = vertical_pos;
        self
    }

    pub fn alignment(
        self,
        horizontal_align: HorizontalAlignment,
    ) -> DrawBuilder<'a, T, C, HorizontalAlignment> {
        DrawBuilder {
            content: self.content,
            position: self.position,
            vertical_pos: self.vertical_pos,
            horizontal_align,
            color: self.color,
            font: self.font,
        }
    }
}

impl<'a, T, A> DrawBuilder<'a, T, (), A> {
    pub fn color<Color>(self, color: Color) -> DrawBuilder<'a, T, DrawColor<Color>, A> {
        DrawBuilder {
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

impl<T, Color, A> DrawBuilder<'_, T, DrawColor<Color>, A> {
    pub fn color(mut self, color: Color) -> Self {
        self.color.fg = color;
        self
    }

    pub fn background(mut self, color: Color) -> Result<Self, LookupError> {
        if self.font.supports_background_color {
            self.color.bg = Some(color);
            Ok(self)
        } else {
            Err(LookupError::BackgroundColorNotSupported)
        }
    }
}

impl<T, Color> DrawBuilder<'_, T, DrawColor<Color>, ()>
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

impl<T, Color> DrawBuilder<'_, T, DrawColor<Color>, HorizontalAlignment>
where
    T: Content,
{
    pub fn draw<Display>(
        &self,
        display: &mut Display,
    ) -> Result<Option<Rectangle>, Error<Display::Error>>
    where
        Display: DrawTarget<Color = Color>,
        Display::Error: core::fmt::Debug,
    {
        draw::draw_aligned(self, display)
    }
}

impl<T, C> DrawBuilder<'_, T, C, ()>
where
    T: Content,
{
    pub fn compute_dimensions(&self) -> Result<RenderedDimensions, LookupError> {
        compute_dimensions::compute_dimensions_unaligned(self)
    }
}

impl<T, C> DrawBuilder<'_, T, C, HorizontalAlignment>
where
    T: Content,
{
    pub fn compute_dimensions(&self) -> Result<Option<Rectangle>, LookupError> {
        compute_dimensions::compute_dimensions_aligned(self)
    }
}
