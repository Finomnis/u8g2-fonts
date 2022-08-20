use embedded_graphics_core::{
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};

use crate::{
    font_reader::FontReader,
    types::{HorizontalAlignment, RenderedDimensions, VerticalPosition},
    utils::combine_bounding_boxes,
    DrawError, Error, Font,
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
    /// * `foreground_color` - The foreground color.
    /// * `background_color` - The background color.
    /// * `vertical_pos` - The vertical positioning.
    /// * `display` - The display to render to.
    ///
    /// # Return
    ///
    /// The pixel advance of the rendered glyph, indicating the required offset to render the next character.
    ///
    pub fn render_glyph<Display>(
        &self,
        ch: char,
        position: Point,
        foreground_color: Display::Color,
        background_color: Option<Display::Color>,
        vertical_pos: VerticalPosition,
        display: &mut Display,
    ) -> Result<i8, DrawError<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        if background_color.is_some() && !self.font.supports_background_color {
            return Err(DrawError::BackgroundColorNotSupported);
        }

        let glyph = self.font.retrieve_glyph_data(ch)?;

        let advance = glyph.advance();
        let size = glyph.size();

        if size.width > 0 && size.height > 0 {
            let renderer = glyph.create_renderer(&self.font);
            if let Some(background_color) = background_color {
                renderer.render_as_box_fill(
                    position,
                    vertical_pos,
                    display,
                    foreground_color,
                    background_color,
                )?;
            } else {
                renderer.render_transparent(position, vertical_pos, display, foreground_color)?;
            }
        }

        Ok(advance)
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
    /// * `foreground_color` - The foreground color.
    /// * `background_color` - The background color.
    /// * `vertical_pos` - The vertical positioning.
    /// * `display` - The display to render to.
    ///
    /// # Return
    ///
    /// The total pixel advance of all rendered glyphs.
    /// Two-dimensional, as newlines change the y position.
    ///
    pub fn render_text<Display>(
        &self,
        text: &str,
        position: Point,
        foreground_color: Display::Color,
        background_color: Option<Display::Color>,
        vertical_pos: VerticalPosition,
        display: &mut Display,
    ) -> Result<Point, DrawError<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let mut advance = Point::new(0, 0);

        for ch in text.chars() {
            if ch == '\n' {
                advance.x = 0;
                advance.y += self.font.font_bounding_box_height as i32 + 1;
            } else {
                advance.x += self.render_glyph(
                    ch,
                    position + advance,
                    foreground_color,
                    background_color,
                    vertical_pos,
                    display,
                )? as i32;
            }
        }

        Ok(advance)
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
    /// * `foreground_color` - The foreground color.
    /// * `background_color` - The background color.
    /// * `vertical_pos` - The vertical positioning.
    /// * `horizontal_align` - The horizontal positioning.
    /// * `display` - The display to render to.
    ///
    pub fn render_text_aligned<Display>(
        &self,
        text: &str,
        mut position: Point,
        foreground_color: Display::Color,
        background_color: Option<Display::Color>,
        vertical_pos: VerticalPosition,
        horizontal_align: HorizontalAlignment,
        display: &mut Display,
    ) -> Result<(), DrawError<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let num_lines = text.lines().count();
        let newline_advance = self.font.font_bounding_box_height as i32 + 1;
        let ascent = self.font.ascent as i32;
        let descent = self.font.descent as i32;

        if num_lines == 0 {
            return Ok(());
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

        for (line_num, line) in text.lines().enumerate() {
            let offset_x = if let HorizontalAlignment::Left = horizontal_align {
                // Alignment: Left

                // From experiments, it seems that alignment looks more symmetrical
                // if everything is shifted by one in respect to the anchor point
                1
            } else {
                // Pre-render to determine
                let dimensions =
                    self.get_text_dimensions(line, Point::new(0, 0), VerticalPosition::default())?;

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

            self.render_text(
                line,
                position + Point::new(offset_x, line_num as i32 * newline_advance),
                foreground_color,
                background_color,
                VerticalPosition::Baseline,
                display,
            )?;
        }

        Ok(())
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
    /// The pixel advance of the rendered glyph, indicating the required offset to render the next character.
    ///
    pub fn get_glyph_dimensions(
        &self,
        ch: char,
        position: Point,
        vertical_pos: VerticalPosition,
    ) -> Result<RenderedDimensions, Error> {
        let glyph = self.font.retrieve_glyph_data(ch)?;

        let advance = glyph.advance();
        let size = glyph.size();

        let bounding_box = (size.width > 0 && size.height > 0).then(|| {
            glyph
                .create_renderer(&self.font)
                .get_glyph_bounding_box(position, vertical_pos)
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
    /// The pixel advance of the rendered text, indicating the required offset to render the next character.
    ///
    pub fn get_text_dimensions(
        &self,
        text: &str,
        position: Point,
        vertical_pos: VerticalPosition,
    ) -> Result<RenderedDimensions, Error> {
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
