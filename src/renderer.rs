use embedded_graphics_core::{
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};

use crate::{
    font_reader::FontReader,
    types::{FontColor, HorizontalAlignment, RenderedDimensions, VerticalPosition},
    Error, Font, LookupError, Renderable,
};

/// Renders text of a specific [`Font`] to a [`DrawTarget`].
#[derive(Debug)]
pub struct FontRenderer {
    font: FontReader,
}

impl FontRenderer {
    /// Creates a new instance of a font renderer.
    ///
    /// # Generics
    ///
    /// * `FONT` - the font to render. See [fonts](crate::fonts) for a list of available fonts
    ///            and refer to [U8g2](https://github.com/olikraus/u8g2/wiki/fntlistall) for a more detailed description of each font.
    pub const fn new<FONT: Font>() -> Self {
        Self {
            font: FontReader::new::<FONT>(),
        }
    }

    /// Renders text to a display.
    ///
    /// Note that the background color is optional. Omitting it will render
    /// the string with a transparent background.
    ///
    /// Not every font supports a background color, some fonts require a transparent background.
    ///
    /// # Arguments
    ///
    /// * `content` - The text/character to render.
    /// * `position` - The position to render to.
    /// * `color` - The font color.
    /// * `vertical_pos` - The vertical positioning.
    /// * `display` - The display to render to.
    ///
    /// # Return
    ///
    /// The dimensions of the rendered text.
    /// The advance might be two-dimensional, as newlines change the y position.
    ///
    pub fn render<Display>(
        &self,
        content: impl Renderable,
        position: Point,
        vertical_pos: VerticalPosition,
        color: FontColor<Display::Color>,
        display: &mut Display,
    ) -> Result<RenderedDimensions, Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        todo!()
    }

    /// Renders text to a display with horizontal and vertical alignment.
    ///
    /// Vertical alignment here means that multi-line strings will anchor properly, compared to [`render_text()`](crate::FontRenderer::render_text),
    /// which always anchors on the first line.
    ///
    /// Note that this function is most likely a little bit slower than [`render_text()`](crate::FontRenderer::render_text), so prefer [`render_text()`](crate::FontRenderer::render_text)
    /// for left-aligned single-line strings.
    ///
    /// # Arguments
    ///
    /// * `content` - The text/character to render.
    /// * `position` - The position to render to.
    /// * `color` - The font color.
    /// * `vertical_pos` - The vertical positioning.
    /// * `horizontal_align` - The horizontal positioning.
    /// * `display` - The display to render to.
    ///
    /// # Return
    ///
    /// The bounding box of the rendered text.
    ///
    /// Does not return an advance value like the other methods,
    /// as due to the alignment it would be meaningless.
    ///
    ///
    pub fn render_aligned<Display>(
        &self,
        content: impl Renderable,
        mut position: Point,
        vertical_pos: VerticalPosition,
        horizontal_align: HorizontalAlignment,
        color: FontColor<Display::Color>,
        display: &mut Display,
    ) -> Result<Option<Rectangle>, Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        todo!()
    }

    /// Calculates the dimensions that rendering text with [`render()`](crate::FontRenderer::render) would produce.
    ///
    /// # Arguments
    ///
    /// * `content` - The text/character to render.
    /// * `position` - The position to render to.
    /// * `vertical_pos` - The vertical positioning.
    ///
    /// # Return
    ///
    /// The dimensions of the rendered text.
    ///
    pub fn get_rendered_dimensions(
        &self,
        content: impl Renderable,
        position: Point,
        vertical_pos: VerticalPosition,
    ) -> Result<RenderedDimensions, LookupError> {
        todo!()
    }

    /// Calculates the dimensions that rendering text with
    /// [`render_ligned()`](crate::FontRenderer::render_aligned) would produce.
    ///
    /// # Arguments
    ///
    /// * `content` - The text/character to render.
    /// * `position` - The position to render to.
    /// * `vertical_pos` - The vertical positioning.
    /// * `horizontal_align` - The horizontal alignment.
    ///
    /// # Return
    ///
    /// The bounding box of the rendered text
    ///
    pub fn get_rendered_dimensions_aligned(
        &self,
        content: impl Renderable,
        mut position: Point,
        vertical_pos: VerticalPosition,
        horizontal_align: HorizontalAlignment,
    ) -> Result<Option<Rectangle>, LookupError> {
        todo!()
    }

    /// The ascent of the font.
    ///
    /// Usually a positive number.
    pub fn get_ascent(&self) -> i8 {
        self.font.ascent
    }

    /// The descent of the font.
    ///
    /// *IMPORTANT*: This is usually a *negative* number.
    pub fn get_descent(&self) -> i8 {
        self.font.descent
    }

    /// The maximum possible bounding box of a glyph if it was rendered with its baseline at (0,0).
    pub fn get_glyph_bounding_box(&self) -> Rectangle {
        Rectangle {
            top_left: Point::new(
                self.font.font_bounding_box_x_offset as i32,
                -(self.font.font_bounding_box_height as i32
                    + self.font.font_bounding_box_y_offset as i32),
            ),
            size: Size::new(
                self.font.font_bounding_box_width as u32,
                self.font.font_bounding_box_height as u32,
            ),
        }
    }
}
