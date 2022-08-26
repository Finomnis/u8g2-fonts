use crate::{utils::DebugIgnore, Font, LookupError};

use self::{glyph_reader::GlyphReader, glyph_searcher::GlyphSearcher};

mod glyph_reader;
mod glyph_renderer;
mod glyph_searcher;
mod unicode_jumptable_reader;

#[derive(Debug)]
pub struct FontReader {
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
    pub ascent_of_parantheses: i8,
    pub descent_of_parantheses: i8,
    pub array_offset_upper_a: u16,
    pub array_offset_lower_a: u16,
    pub array_offset_0x0100: u16,
}

impl FontReader {
    pub const fn new<F: Font>() -> Self {
        let data = F::DATA;

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
            ascent_of_parantheses: data[15] as i8,
            descent_of_parantheses: data[16] as i8,
            array_offset_upper_a: u16::from_be_bytes([data[17], data[18]]),
            array_offset_lower_a: u16::from_be_bytes([data[19], data[20]]),
            array_offset_0x0100: u16::from_be_bytes([data[21], data[22]]),
        }
    }

    pub fn retrieve_glyph_data(&self, ch: char) -> Result<GlyphReader, LookupError> {
        // Retrieve u16 glyph value
        let encoding = u16::try_from(ch as u32).map_err(|_| LookupError::GlyphNotFound(ch))?;

        let mut glyph = GlyphSearcher::new(self);

        if encoding <= 255 {
            if encoding >= b'a' as u16 {
                glyph
                    .jump_by(self.array_offset_lower_a as usize)
                    .then_some(())
                    .ok_or(LookupError::GlyphNotFound(ch))?;
            } else if encoding >= b'A' as u16 {
                glyph
                    .jump_by(self.array_offset_upper_a as usize)
                    .then_some(())
                    .ok_or(LookupError::GlyphNotFound(ch))?;
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
