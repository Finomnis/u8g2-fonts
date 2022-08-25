use core::fmt::Arguments;

use embedded_graphics_core::{
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};

use crate::{
    draw_builder::content::ArgsContent,
    font_reader::FontReader,
    types::{FontColor, HorizontalAlignment, RenderedDimensions, VerticalPosition},
    utils::{combine_bounding_boxes, FormatArgsReader},
    DrawBuilder, Error, Font, LookupError,
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

    /// Renders a character glyph.
    ///
    /// Note that the background color is optional. Omitting it will render
    /// the character with a transparent background.
    ///
    /// Not every font supports a background color, some fonts require a transparent background.
    ///
    /// # Arguments
    ///
    /// * `ch` - The character to render.
    /// * `position` - The position to render to.
    /// * `color` - The font color.
    /// * `vertical_pos` - The vertical positioning.
    /// * `display` - The display to render to.
    ///
    /// # Return
    ///
    /// The dimensions of the rendered glyph.
    /// The y component of the advance will always be zero.
    ///
    pub fn render_glyph<Display>(
        &self,
        ch: char,
        position: Point,
        color: FontColor<Display::Color>,
        vertical_pos: VerticalPosition,
        display: &mut Display,
    ) -> Result<RenderedDimensions, Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let glyph = self.font.retrieve_glyph_data(ch)?;

        let advance = glyph.advance();
        let size = glyph.size();

        let bounding_box = if size.width > 0 && size.height > 0 {
            let renderer = glyph.create_renderer(&self.font);
            Some(match color {
                FontColor::Transparent(color) => {
                    renderer.render_transparent(position, display, color)?
                }
                FontColor::WithBackground { fg, bg } => {
                    renderer.render_as_box_fill(position, display, fg, bg)?
                }
            })
        } else {
            None
        };

        Ok(RenderedDimensions {
            advance: Point::new(advance as i32, 0),
            bounding_box,
        })
    }

    /// Renders a string.
    ///
    /// Note that the background color is optional. Omitting it will render
    /// the string with a transparent background.
    ///
    /// Not every font supports a background color, some fonts require a transparent background.
    ///
    /// # Arguments
    ///
    /// * `text` - The string to render.
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
    pub fn render_text<Display>(
        &self,
        text: &str,
        position: Point,
        color: FontColor<Display::Color>,
        vertical_pos: VerticalPosition,
        display: &mut Display,
    ) -> Result<RenderedDimensions, Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let mut advance = Point::new(0, 0);

        let mut bounding_box = None;

        for ch in text.chars() {
            if ch == '\n' {
                advance.x = 0;
                advance.y += self.font.font_bounding_box_height as i32 + 1;
            } else {
                let dimensions =
                    self.render_glyph(ch, position + advance, color, vertical_pos, display)?;
                advance += dimensions.advance;
                bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
            }
        }

        Ok(RenderedDimensions {
            advance,
            bounding_box,
        })
    }

    /// Renders a string with horizontal and vertical alignment.
    ///
    /// Vertical alignment here means that multi-line strings will anchor properly, compared to [`render_text()`](crate::FontRenderer::render_text),
    /// which always anchors on the first line.
    ///
    /// Note that this function is most likely a little bit slower than [`render_text()`](crate::FontRenderer::render_text), so prefer [`render_text()`](crate::FontRenderer::render_text)
    /// for left-aligned single-line strings.
    ///
    /// # Arguments
    ///
    /// * `text` - The string to render.
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
    pub fn render_text_aligned<Display>(
        &self,
        text: &str,
        mut position: Point,
        color: FontColor<Display::Color>,
        vertical_pos: VerticalPosition,
        horizontal_align: HorizontalAlignment,
        display: &mut Display,
    ) -> Result<Option<Rectangle>, Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let num_lines = text.lines().count();
        let newline_advance = self.font.font_bounding_box_height as i32 + 1;
        let ascent = self.font.ascent as i32;
        let descent = self.font.descent as i32;

        if num_lines == 0 {
            return Ok(None);
        }

        let vertical_offset = match vertical_pos {
            VerticalPosition::Baseline => 0,
            VerticalPosition::Top => ascent + 1,
            VerticalPosition::Center => {
                let total_newline_advance = (num_lines - 1) as i32 * newline_advance;
                (total_newline_advance + ascent - descent + 1) / 2 + descent - total_newline_advance
            }
            VerticalPosition::Bottom => descent - (num_lines - 1) as i32 * newline_advance,
        };
        position.y += vertical_offset;

        let mut bounding_box = None;

        for (line_num, line) in text.lines().enumerate() {
            let offset_x = if let HorizontalAlignment::Left = horizontal_align {
                // Alignment: Left

                // From experiments, it seems that alignment looks more symmetrical
                // if everything is shifted by one in respect to the anchor point
                1
            } else {
                // Pre-render to determine
                let dimensions =
                    self.get_text_dimensions(line, Point::new(0, 0), VerticalPosition::Baseline)?;

                if let HorizontalAlignment::Center = horizontal_align {
                    // Alignment: Center
                    if let Some(bounding_box) = dimensions.bounding_box {
                        let width = bounding_box.size.width;
                        let left = bounding_box.top_left.x;

                        -(width as i32 / 2 + left)
                    } else {
                        0
                    }
                } else {
                    // Alignment: Right

                    // From experiments, it seems that alignment looks more symmetrical
                    // if everything is shifted by one in respect to the anchor point
                    1 - dimensions.advance.x
                }
            };

            let dimensions = self.render_text(
                line,
                position + Point::new(offset_x, line_num as i32 * newline_advance),
                color,
                VerticalPosition::Baseline,
                display,
            )?;

            bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
        }

        Ok(bounding_box)
    }

    /// Renders format string arguments.
    ///
    /// Apart of being able to render format strings, this function is identical
    /// to [`render_text()`](crate::FontRenderer::render_text).
    ///
    /// # Arguments
    ///
    /// * `args` - The format string arguments to render.
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
    pub fn render_args<Display>(
        &self,
        args: Arguments<'_>,
        position: Point,
        color: FontColor<Display::Color>,
        vertical_pos: VerticalPosition,
        display: &mut Display,
    ) -> Result<RenderedDimensions, Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let mut advance = Point::new(0, 0);

        let mut bounding_box = None;

        FormatArgsReader::new(|ch| -> Result<(), Error<Display::Error>> {
            if ch == '\n' {
                advance.x = 0;
                advance.y += self.font.font_bounding_box_height as i32 + 1;
            } else {
                let dimensions =
                    self.render_glyph(ch, position + advance, color, vertical_pos, display)?;
                advance += dimensions.advance;
                bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
            }
            Ok(())
        })
        .process_args(args)?;

        Ok(RenderedDimensions {
            advance,
            bounding_box,
        })
    }

    /// Calculates the dimensions that rendering a glyph with [`render_glyph()`](crate::FontRenderer::render_glyph) would produce.
    ///
    /// # Arguments
    ///
    /// * `ch` - The character to render.
    /// * `position` - The position to render to.
    /// * `vertical_pos` - The vertical positioning.
    ///
    /// # Return
    ///
    /// The dimensions of the rendered glyph
    ///
    pub fn get_glyph_dimensions(
        &self,
        ch: char,
        position: Point,
        vertical_pos: VerticalPosition,
    ) -> Result<RenderedDimensions, LookupError> {
        let glyph = self.font.retrieve_glyph_data(ch)?;

        let advance = glyph.advance();
        let size = glyph.size();

        let bounding_box = (size.width > 0 && size.height > 0).then(|| {
            glyph
                .create_renderer(&self.font)
                .get_glyph_bounding_box(position)
        });

        Ok(RenderedDimensions {
            advance: Point::new(advance as i32, 0),
            bounding_box,
        })
    }

    /// Calculates the dimensions that rendering a text with [`render_text()`](crate::FontRenderer::render_text) would produce.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to render.
    /// * `position` - The position to render to.
    /// * `vertical_pos` - The vertical positioning.
    ///
    /// # Return
    ///
    /// The dimensions if the rendered text.
    ///
    pub fn get_text_dimensions(
        &self,
        text: &str,
        position: Point,
        vertical_pos: VerticalPosition,
    ) -> Result<RenderedDimensions, LookupError> {
        let mut advance = Point::new(0, 0);
        let mut bounding_box = None;

        for ch in text.chars() {
            if ch == '\n' {
                advance.x = 0;
                advance.y += self.font.font_bounding_box_height as i32 + 1;
            } else {
                let dimensions = self.get_glyph_dimensions(ch, position + advance, vertical_pos)?;

                advance += dimensions.advance;
                bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
            }
        }

        Ok(RenderedDimensions {
            advance,
            bounding_box,
        })
    }

    /// Calculates the dimensions that rendering a text with
    /// [`render_text_aligned()`](crate::FontRenderer::render_text_aligned) would produce.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to render.
    /// * `position` - The position to render to.
    /// * `vertical_pos` - The vertical positioning.
    /// * `horizontal_align` - The horizontal alignment.
    ///
    /// # Return
    ///
    /// The bounding box of the rendered text
    ///
    pub fn get_aligned_text_dimensions(
        &self,
        text: &str,
        mut position: Point,
        vertical_pos: VerticalPosition,
        horizontal_align: HorizontalAlignment,
    ) -> Result<Option<Rectangle>, LookupError> {
        let num_lines = text.lines().count();
        let newline_advance = self.font.font_bounding_box_height as i32 + 1;
        let ascent = self.font.ascent as i32;
        let descent = self.font.descent as i32;

        if num_lines == 0 {
            return Ok(None);
        }

        let vertical_offset = match vertical_pos {
            VerticalPosition::Baseline => 0,
            VerticalPosition::Top => ascent + 1,
            VerticalPosition::Center => {
                let total_newline_advance = (num_lines - 1) as i32 * newline_advance;
                (total_newline_advance + ascent - descent + 1) / 2 + descent - total_newline_advance
            }
            VerticalPosition::Bottom => descent - (num_lines - 1) as i32 * newline_advance,
        };
        position.y += vertical_offset;

        let mut bounding_box = None;

        for (line_num, line) in text.lines().enumerate() {
            // Pre-render to determine
            let dimensions =
                self.get_text_dimensions(line, Point::new(0, 0), VerticalPosition::Baseline)?;

            let offset_x = match horizontal_align {
                HorizontalAlignment::Left => {
                    // From experiments, it seems that alignment looks more symmetrical
                    // if everything is shifted by one in respect to the anchor point
                    1
                }
                HorizontalAlignment::Center => {
                    if let Some(bounding_box) = dimensions.bounding_box {
                        let width = bounding_box.size.width;
                        let left = bounding_box.top_left.x;

                        -(width as i32 / 2 + left)
                    } else {
                        0
                    }
                }
                HorizontalAlignment::Right => {
                    // From experiments, it seems that alignment looks more symmetrical
                    // if everything is shifted by one in respect to the anchor point
                    1 - dimensions.advance.x
                }
            };

            bounding_box = combine_bounding_boxes(
                bounding_box,
                dimensions.bounding_box.map(|mut d| {
                    d.top_left +=
                        position + Point::new(offset_x, line_num as i32 * newline_advance);
                    d
                }),
            );
        }

        Ok(bounding_box)
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

    pub fn render_args_with_builder<'a>(
        &'a self,
        args: Arguments<'a>,
    ) -> DrawBuilder<ArgsContent<'a>, ()> {
        DrawBuilder::from_args(&self.font, args)
    }
}
