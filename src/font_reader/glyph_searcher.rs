use crate::font_reader::{
    glyph_reader::GlyphReader, unicode_jumptable_reader::UnicodeJumptableReader, FontReader,
};

pub struct GlyphSearcher<'a, const CHAR_WIDTH: usize> {
    data: &'static [u8],
    font: &'a FontReader,
}

impl<const CHAR_WIDTH: usize> GlyphSearcher<'_, CHAR_WIDTH> {
    pub fn jump_by(&mut self, offset: usize) {
        self.data = &self.data[offset..];
    }

    fn get_offset(&self) -> u8 {
        self.data.get(CHAR_WIDTH).cloned().unwrap()
    }

    pub fn jump_to_next(&mut self) -> bool {
        let offset = self.get_offset();
        if offset == 0 {
            false
        } else {
            self.jump_by(offset.into());
            true
        }
    }

    pub fn into_glyph_reader(self) -> GlyphReader {
        GlyphReader::new(self.data.get((CHAR_WIDTH + 1)..).unwrap(), self.font)
    }
}

const U8G2_FONT_DATA_STRUCT_SIZE: usize = 23;

/// Walks the jump chain of the single byte encoding region and returns the offset
/// of the glyph of `encoding`, relative to the start of the glyph data.
///
/// The returned value is exactly the value that has to be passed to
/// [`GlyphSearcher::jump_by`] to make a freshly created [`GlyphSearcher<1>`]
/// point at the glyph of `encoding`.
///
/// Returns [`None`] if the font does not contain `encoding`.
pub const fn find_glyph_offset(
    data: &[u8],
    array_offset_upper_a: u16,
    array_offset_lower_a: u16,
    encoding: u8,
) -> Option<usize> {
    let mut offset = if encoding >= b'a' {
        array_offset_lower_a as usize
    } else if encoding >= b'A' {
        array_offset_upper_a as usize
    } else {
        0
    };

    loop {
        let pos = U8G2_FONT_DATA_STRUCT_SIZE + offset;

        if pos + 1 >= data.len() {
            return None;
        }

        // The terminator of the chain is not a glyph, so it is checked first.
        let jump_distance = data[pos + 1];
        if jump_distance == 0 {
            return None;
        }
        if data[pos] == encoding {
            return Some(offset);
        }

        offset += jump_distance as usize;
    }
}

impl<'a> GlyphSearcher<'a, 1> {
    pub fn new(font: &'a FontReader) -> Self {
        Self {
            data: &font.data[U8G2_FONT_DATA_STRUCT_SIZE..],
            font,
        }
    }

    pub fn into_unicode_mode(
        mut self,
        offset: u16,
    ) -> (GlyphSearcher<'a, 2>, UnicodeJumptableReader) {
        self.jump_by(offset.into());

        (
            GlyphSearcher {
                data: self.data,
                font: self.font,
            },
            UnicodeJumptableReader::new(self.data),
        )
    }
}

impl GlyphSearcher<'_, 2> {
    pub fn get_ch(&self) -> u16 {
        u16::from_be_bytes([
            self.data.first().cloned().unwrap(),
            self.data.get(1).cloned().unwrap(),
        ])
    }
}
