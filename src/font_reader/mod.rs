use crate::{utils::DebugIgnore, FontData, LookupError};

use self::{glyph_reader::GlyphReader, glyph_searcher::GlyphSearcher};

mod cursor;
mod glyph_reader;
mod glyph_renderer;
mod glyph_searcher;
mod unicode_jumptable_reader;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FontReader<'a, D: ?Sized> {
    pub data: DebugIgnore<&'a D>,
    pub glyph_count: u8,
    pub supports_background_color: bool,
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
    pub line_height: u32,
}

impl<'a, D: FontData> FontReader<'a, D> {
    pub fn new(
        data: &'a D,
        ignore_unknown_glyphs: bool,
        line_height_override: Option<u32>,
    ) -> Self {
        let mut this = Self {
            data: DebugIgnore(data),
            glyph_count: data.get_byte(0),
            supports_background_color: data.get_byte(1) != 0,
            m0: data.get_byte(2),
            m1: data.get_byte(3),
            bitcnt_w: data.get_byte(4),
            bitcnt_h: data.get_byte(5),
            bitcnt_x: data.get_byte(6),
            bitcnt_y: data.get_byte(7),
            bitcnt_d: data.get_byte(8),
            font_bounding_box_width: data.get_byte(9) as i8,
            font_bounding_box_height: data.get_byte(10) as i8,
            font_bounding_box_x_offset: data.get_byte(11) as i8,
            font_bounding_box_y_offset: data.get_byte(12) as i8,
            ascent: data.get_byte(13) as i8,
            descent: data.get_byte(14) as i8,
            ascent_of_parentheses: data.get_byte(15) as i8,
            descent_of_parentheses: data.get_byte(16) as i8,
            array_offset_upper_a: u16::from_be_bytes([data.get_byte(17), data.get_byte(18)]),
            array_offset_lower_a: u16::from_be_bytes([data.get_byte(19), data.get_byte(20)]),
            array_offset_0x0100: u16::from_be_bytes([data.get_byte(21), data.get_byte(22)]),
            ignore_unknown_glyphs,
            line_height: 0,
        };
        if let Some(line_height) = line_height_override {
            this.line_height = line_height;
        } else {
            this.line_height = this.get_default_line_height() as u32;
        }
        this
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
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::format;

    use super::*;

    use crate::font::Font;

    struct TestFont;
    impl crate::Font for TestFont {
        const DATA: &'static [u8] = &[
            0, 0, 4, 4, 8, 8, 8, 8, 8, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 2, // Header
            b'\n', 0, // First glyph
            0, 4, 255, 255, // Unicode Table
            0, b'\n', 0, // Unicode entry
        ];
    }

    #[test]
    fn can_read_font_properties() {
        let font = FontReader::new(&TestFont::DATA, false, None);

        let expected = FontReader::<[u8]> {
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
        let font = FontReader::new(&TestFont::DATA, false, None);
        let glyph = font.retrieve_glyph_data('☃');

        assert!(matches!(glyph, Err(LookupError::GlyphNotFound('☃'))));
    }
}
