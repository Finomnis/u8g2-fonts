use crate::{utils::DebugIgnore, LookupError};

use self::{glyph_reader::GlyphReader, glyph_searcher::GlyphSearcher};

use embedded_graphics_core::{
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};

use crate::{
    content::{
        vertical_offset::compute_vertical_offset_from_static_newlines, LineDimensionsIterator,
    },
    types::{FontColor, HorizontalAlignment, RenderedDimensions, VerticalPosition},
    utils::{combine_bounding_boxes, HorizontalRenderedDimensions},
    Content, Error,
};

use crate::render_actions::{compute_glyph_dimensions, compute_horizontal_offset, render_glyph};

mod glyph_reader;
mod glyph_renderer;
mod glyph_searcher;
mod unicode_jumptable_reader;

#[derive(Debug, Clone)]
pub struct Font {
    pub data: DebugIgnore<&'static [u8]>,
    pub supports_background_color: bool,
    pub glyph_count: u8,
    pub m0: u8,
    pub m1: u8,
    pub bitcnt_w: u8,
    pub bitcnt_h: u8,
    pub bitcnt_x: u8,
    pub bitcnt_y: u8,
    pub bitcnt_d: u8,
    pub font_bounding_box_width: i8,
    pub font_bounding_box_height: i8,
    pub font_bounding_box_x_offset: i8,
    pub font_bounding_box_y_offset: i8,
    pub ascent: i8,
    pub descent: i8,
    pub ascent_of_parentheses: i8,
    pub descent_of_parentheses: i8,
    pub array_offset_upper_a: u16,
    pub array_offset_lower_a: u16,
    pub array_offset_0x0100: u16,
    pub ignore_unknown_glyphs: bool,
    pub line_height: u8,
}

impl Font {
    pub const fn new(f: &'static [u8]) -> Self {
        let data = f;

        Self {
            data: DebugIgnore(data),
            glyph_count: data[0],
            supports_background_color: data[1] != 0,
            m0: data[2],
            m1: data[3],
            bitcnt_w: data[4],
            bitcnt_h: data[5],
            bitcnt_x: data[6],
            bitcnt_y: data[7],
            bitcnt_d: data[8],
            font_bounding_box_width: data[9] as i8,
            font_bounding_box_height: data[10] as i8,
            font_bounding_box_x_offset: data[11] as i8,
            font_bounding_box_y_offset: data[12] as i8,
            ascent: data[13] as i8,
            descent: data[14] as i8,
            ascent_of_parentheses: data[15] as i8,
            descent_of_parentheses: data[16] as i8,
            array_offset_upper_a: u16::from_be_bytes([data[17], data[18]]),
            array_offset_lower_a: u16::from_be_bytes([data[19], data[20]]),
            array_offset_0x0100: u16::from_be_bytes([data[21], data[22]]),
            ignore_unknown_glyphs: false,
            line_height: data[10] + 1,
        }
    }

    pub const fn with_ignore_unknown_glyphs(mut self, ignore: bool) -> Self {
        self.ignore_unknown_glyphs = ignore;
        self
    }

    pub const fn with_line_height(mut self, line_height: u8) -> Self {
        self.line_height = line_height;
        self
    }

    pub const fn get_default_line_height(&self) -> u8 {
        assert!(self.font_bounding_box_height >= 0);
        self.font_bounding_box_height as u8 + 1
    }

    pub fn try_retrieve_glyph_data(&self, ch: char) -> Result<Option<GlyphReader>, LookupError> {
        match self.retrieve_glyph_data(ch) {
            Err(LookupError::GlyphNotFound(_)) if self.ignore_unknown_glyphs => Ok(None),
            Ok(g) => Ok(Some(g)),
            Err(e) => Err(e),
        }
    }

    fn retrieve_glyph_data(&self, ch: char) -> Result<GlyphReader, LookupError> {
        // Retrieve u16 glyph value
        let encoding = u16::try_from(u32::from(ch)).map_err(|_| LookupError::GlyphNotFound(ch))?;

        let mut glyph = GlyphSearcher::new(self);

        if encoding <= 255 {
            if encoding >= u16::from(b'a') {
                glyph.jump_by(self.array_offset_lower_a.into());
            } else if encoding >= u16::from(b'A') {
                glyph.jump_by(self.array_offset_upper_a.into());
            }

            while glyph.get_ch() as u16 != encoding {
                glyph
                    .jump_to_next()
                    .then_some(())
                    .ok_or(LookupError::GlyphNotFound(ch))?;
            }

            Ok(glyph.into_glyph_reader())
        } else {
            let (mut glyph, unicode_jump_table) = glyph.into_unicode_mode(self.array_offset_0x0100);

            let jump_offset = unicode_jump_table
                .calculate_jump_offset(encoding)
                .ok_or(LookupError::GlyphNotFound(ch))?;

            glyph.jump_by(jump_offset);

            loop {
                let glyph_ch = glyph.get_ch();
                if glyph_ch == 0 {
                    return Err(LookupError::GlyphNotFound(ch));
                }
                if glyph_ch == encoding {
                    break;
                }
                if !glyph.jump_to_next() {
                    return Err(LookupError::GlyphNotFound(ch));
                }
            }

            Ok(glyph.into_glyph_reader())
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
        self = self.with_ignore_unknown_glyphs(ignore);
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
        let font = &self;
        if color.has_background() && !font.supports_background_color {
            return Err(Error::BackgroundColorNotSupported);
        }

        let mut advance = Point::new(0, 0);

        let mut bounding_box = None;

        position.y += content.compute_vertical_offset(font, vertical_pos);

        content.for_each_char(|ch| -> Result<(), Error<Display::Error>> {
            if ch == '\n' {
                advance.x = 0;
                advance.y += i32::from(font.line_height);
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

        let font = &self;
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
                advance.y += i32::from(font.line_height);
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
        let font = &self;

        let mut advance = Point::new(0, 0);

        let mut bounding_box = None;

        position.y += content.compute_vertical_offset(font, vertical_pos);

        content.for_each_char(|ch| -> Result<(), LookupError> {
            if ch == '\n' {
                advance.x = 0;
                advance.y += i32::from(font.line_height);
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
        let font = &self;

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
                position.y += i32::from(font.line_height);
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
        self.ascent
    }

    /// The descent of the font.
    ///
    /// *IMPORTANT*: This is usually a *negative* number.
    pub const fn get_descent(&self) -> i8 {
        self.descent
    }

    /// The maximum possible bounding box of all glyphs if they were rendered with
    /// [`render()`](crate::FontRenderer::render) at position `(0,0)`.
    pub const fn get_font_bounding_box(&self, vertical_pos: VerticalPosition) -> Rectangle {
        let y_offset = compute_vertical_offset_from_static_newlines(self, vertical_pos, 0);
        Rectangle {
            top_left: Point::new(
                self.font_bounding_box_x_offset as i32,
                y_offset
                    - (self.font_bounding_box_height as i32
                        + self.font_bounding_box_y_offset as i32),
            ),
            size: Size::new(
                self.font_bounding_box_width as u32,
                self.font_bounding_box_height as u32,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::format;

    use super::*;

    const TEST_FONT: &'static [u8] = &[
        0, 0, 4, 4, 8, 8, 8, 8, 8, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 2, // Header
        b'\n', 0, // First glyph
        0, 4, 255, 255, // Unicode Table
        0, b'\n', 0, // Unicode entry
    ];

    #[test]
    fn can_read_font_properties() {
        let font = Font::new(TEST_FONT);

        let expected = Font {
            data: DebugIgnore(&[]),
            supports_background_color: false,
            glyph_count: 0,
            m0: 4,
            m1: 4,
            bitcnt_w: 8,
            bitcnt_h: 8,
            bitcnt_x: 8,
            bitcnt_y: 8,
            bitcnt_d: 8,
            font_bounding_box_width: 1,
            font_bounding_box_height: 2,
            font_bounding_box_x_offset: 3,
            font_bounding_box_y_offset: 4,
            ascent: 5,
            descent: 6,
            ascent_of_parentheses: 7,
            descent_of_parentheses: 8,
            array_offset_upper_a: 0,
            array_offset_lower_a: 0,
            array_offset_0x0100: 2,
            ignore_unknown_glyphs: false,
            line_height: 3,
        };

        assert_eq!(format!("{font:?}"), format!("{expected:?}"));
    }

    #[test]
    fn can_handle_unicode_next_is_zero() {
        // This test is specifically engineered to test an error path that doesn't happen
        // in normal, correct fonts.
        // This means that this should be an assert instead, but it just doesn't feel right.
        // There is no formal specification that this error path is impossible, and resilient
        // programming tells me it should be a normal error path.
        // Sadly, that reduces our test coverage :D so let's trigger that error manually.
        let font = Font::new(TEST_FONT);
        let glyph = font.retrieve_glyph_data('☃');

        assert!(matches!(glyph, Err(LookupError::GlyphNotFound('☃'))));
    }
}
