use embedded_graphics_core::{
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};

use crate::{
    content::{
        vertical_offset::compute_vertical_offset_from_static_newlines, LineDimensionsIterator,
    },
    font_reader::FontReader,
    types::{FontColor, HorizontalAlignment, RenderedDimensions, VerticalPosition},
    utils::{combine_bounding_boxes, HorizontalRenderedDimensions},
    Content, Error, Font, LookupError,
};

use self::render_actions::{compute_glyph_dimensions, compute_horizontal_offset, render_glyph};

pub mod render_actions;

/// Renders text of a specific [`Font`] to a [`DrawTarget`].
#[derive(Debug, Clone)]
pub struct FontRenderer {
    font: FontReader,
}

impl FontRenderer {
    /// Creates a new instance of a font renderer.
    ///
    /// # Generics
    ///
    /// * `FONT` - the font to render. See [fonts](crate::fonts) for a list of available fonts
    ///   and refer to [U8g2](https://github.com/olikraus/u8g2/wiki/fntlistall) for a more detailed description of each font.
    pub const fn new<FONT: Font>() -> Self {
        Self {
            font: FontReader::new::<FONT>(),
        }
    }

    /// Switches the font rendering mode to ignore all unrenderable characters
    /// instead of raising an error.
    ///
    /// By default, unknown chars will return an error.
    ///
    /// # Arguments
    ///
    /// * `ignore` - Whether unknown characters should be ignored.
    pub const fn with_ignore_unknown_chars(mut self, ignore: bool) -> Self {
        self.font = self.font.with_ignore_unknown_glyphs(ignore);
        self
    }

    /// Sets the line height.
    ///
    /// The line height is defined as the vertical distance between the baseline of two adjacent lines in pixels.
    ///
    /// If the line height is not set explicitly, it will default to the value returned by [`get_default_line_height()`](FontRenderer::get_default_line_height).
    ///
    /// # Arguments
    ///
    /// * `line_height` - The desired line height, in pixels.
    pub const fn with_line_height(mut self, line_height: u32) -> Self {
        self.font = self.font.with_line_height(line_height);
        self
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
        content: impl Content,
        mut position: Point,
        vertical_pos: VerticalPosition,
        color: FontColor<Display::Color>,
        display: &mut Display,
    ) -> Result<RenderedDimensions, Error<Display::Error>>
    where
        Display: DrawTarget,
    {
        let font = &self.font;
        if color.has_background() && !font.supports_background_color {
            return Err(Error::BackgroundColorNotSupported);
        }

        let mut advance = Point::new(0, 0);

        let mut bounding_box = None;

        position.y += content.compute_vertical_offset(font, vertical_pos);

        content.for_each_char(|ch| -> Result<(), Error<Display::Error>> {
            if ch == '\n' {
                advance.x = 0;
                advance.y += i32::try_from(font.line_height).unwrap();
            } else {
                let dimensions = render_glyph(ch, position + advance, color, font, display)?;
                advance += dimensions.advance;
                bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
            }

            Ok(())
        })?;

        Ok(RenderedDimensions {
            advance,
            bounding_box,
        })
    }

    /// Renders text to a display with horizontal alignment.
    ///
    /// The `Left` alignment is identical to [`render()`](crate::FontRenderer::render).
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
    /// Does not return an advance value like [`render()`](crate::FontRenderer::render),
    /// as due to the alignment it would be meaningless.
    ///
    ///
    pub fn render_aligned<Display>(
        &self,
        content: impl Content,
        mut position: Point,
        vertical_pos: VerticalPosition,
        horizontal_align: HorizontalAlignment,
        color: FontColor<Display::Color>,
        display: &mut Display,
    ) -> Result<Option<Rectangle>, Error<Display::Error>>
    where
        Display: DrawTarget,
    {
        // If `horizontal_align` is `Left`, it is identical to
        // `render()`. As `render()` is quite a bit faster,
        // forward this call.
        if let HorizontalAlignment::Left = horizontal_align {
            position.x += compute_horizontal_offset(
                HorizontalAlignment::Left,
                HorizontalRenderedDimensions::empty(),
            );
            return self
                .render(content, position, vertical_pos, color, display)
                .map(|dims| dims.bounding_box);
        }

        // This function is a little more complicated.
        // To properly align horizontally, we need to iterate over every line twice.
        // This is really hard with format_args.
        // Therefore we introduce a line_dimensions_iterator that is almost no overhead for
        // glyphs/lines, but makes it possible to implement the format_args case.

        let font = &self.font;
        if color.has_background() && !font.supports_background_color {
            return Err(Error::BackgroundColorNotSupported);
        }

        position.y += content.compute_vertical_offset(font, vertical_pos);

        let mut bounding_box = None;

        let mut line_dimensions = content.line_dimensions_iterator();
        let mut advance = Point::new(
            compute_horizontal_offset(horizontal_align, line_dimensions.next(font)?),
            0,
        );

        content.for_each_char(|ch| -> Result<(), Error<Display::Error>> {
            if ch == '\n' {
                advance.x =
                    compute_horizontal_offset(horizontal_align, line_dimensions.next(font)?);
                advance.y += i32::try_from(font.line_height).unwrap();
            } else {
                let dimensions = render_glyph(ch, position + advance, color, font, display)?;
                advance += dimensions.advance;
                bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
            }

            Ok(())
        })?;

        Ok(bounding_box)
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
        content: impl Content,
        mut position: Point,
        vertical_pos: VerticalPosition,
    ) -> Result<RenderedDimensions, LookupError> {
        let font = &self.font;

        let mut advance = Point::new(0, 0);

        let mut bounding_box = None;

        position.y += content.compute_vertical_offset(font, vertical_pos);

        content.for_each_char(|ch| -> Result<(), LookupError> {
            if ch == '\n' {
                advance.x = 0;
                advance.y += i32::try_from(font.line_height).unwrap();
            } else {
                let dimensions = compute_glyph_dimensions(ch, position + advance, font)?;
                advance += dimensions.advance;
                bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
            }

            Ok(())
        })?;

        Ok(RenderedDimensions {
            advance,
            bounding_box,
        })
    }

    /// Calculates the dimensions that rendering text with
    /// [`render_aligned()`](crate::FontRenderer::render_aligned) would produce.
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
    /// The bounding box of the rendered text.
    ///
    pub fn get_rendered_dimensions_aligned(
        &self,
        content: impl Content,
        mut position: Point,
        vertical_pos: VerticalPosition,
        horizontal_align: HorizontalAlignment,
    ) -> Result<Option<Rectangle>, LookupError> {
        let font = &self.font;

        position.y += content.compute_vertical_offset(font, vertical_pos);

        let mut bounding_box = None;

        let mut line_advance = 0;
        let mut line_bounding_box: Option<Rectangle> = None;
        content.for_each_char(|ch| -> Result<(), LookupError> {
            if ch == '\n' {
                let horizontal_offset = compute_horizontal_offset(
                    horizontal_align,
                    HorizontalRenderedDimensions {
                        advance: line_advance,
                        bounding_box_width: line_bounding_box.map_or(0, |b| b.size.width),
                        bounding_box_offset: line_bounding_box.map_or(0, |b| b.top_left.x),
                    },
                );

                // 'render' by moving the already known bounding box to the correct position
                if let Some(mut line_bounding_box) = line_bounding_box {
                    line_bounding_box.top_left.x += horizontal_offset;
                    line_bounding_box.top_left += position;
                    bounding_box = combine_bounding_boxes(bounding_box, Some(line_bounding_box));
                }

                line_advance = 0;
                line_bounding_box = None;
                position.y += i32::try_from(font.line_height).unwrap();
            } else {
                let dimensions = compute_glyph_dimensions(ch, Point::new(line_advance, 0), font)?;
                line_bounding_box =
                    combine_bounding_boxes(line_bounding_box, dimensions.bounding_box);
                line_advance += dimensions.advance.x;
            }

            Ok(())
        })?;

        // One last pass, if the string didn't end with a newline
        let horizontal_offset = compute_horizontal_offset(
            horizontal_align,
            HorizontalRenderedDimensions {
                advance: line_advance,
                bounding_box_width: line_bounding_box.map_or(0, |b| b.size.width),
                bounding_box_offset: line_bounding_box.map_or(0, |b| b.top_left.x),
            },
        );

        if let Some(mut line_bounding_box) = line_bounding_box {
            line_bounding_box.top_left.x += horizontal_offset;
            line_bounding_box.top_left += position;
            bounding_box = combine_bounding_boxes(bounding_box, Some(line_bounding_box));
        }

        Ok(bounding_box)
    }

    /// The ascent of the font.
    ///
    /// Usually a positive number.
    pub const fn get_ascent(&self) -> i8 {
        self.font.ascent
    }

    /// The descent of the font.
    ///
    /// *IMPORTANT*: This is usually a *negative* number.
    pub const fn get_descent(&self) -> i8 {
        self.font.descent
    }

    /// The maximum possible bounding box of all glyphs if they were rendered with
    /// [`render()`](crate::FontRenderer::render) at position `(0,0)`.
    pub const fn get_font_bounding_box(&self, vertical_pos: VerticalPosition) -> Rectangle {
        let y_offset = compute_vertical_offset_from_static_newlines(&self.font, vertical_pos, 0);
        Rectangle {
            top_left: Point::new(
                self.font.font_bounding_box_x_offset as i32,
                y_offset
                    - (self.font.font_bounding_box_height as i32
                        + self.font.font_bounding_box_y_offset as i32),
            ),
            size: Size::new(
                self.font.font_bounding_box_width as u32,
                self.font.font_bounding_box_height as u32,
            ),
        }
    }

    /// The default line height.
    ///
    /// The line height is defined as the vertical distance between the baseline of two adjacent lines in pixels.
    pub const fn get_default_line_height(&self) -> u32 {
        self.font.get_default_line_height() as u32
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::println;

    use super::*;

    #[test]
    fn implements_debug() {
        println!(
            "{:?}",
            FontRenderer::new::<crate::fonts::u8g2_font_u8glib_4_tf>()
        );
    }
}
