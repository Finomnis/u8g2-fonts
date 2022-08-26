use crate::font_reader::{
    glyph_reader::GlyphReader, unicode_jumptable_reader::UnicodeJumptableReader, FontReader,
};

#[derive(Debug)]
pub struct GlyphSearcher<'a, const CHAR_WIDTH: usize> {
    data: &'static [u8],
    font: &'a FontReader,
}

impl<'a, const CHAR_WIDTH: usize> GlyphSearcher<'a, CHAR_WIDTH> {
    pub fn jump_by(&mut self, offset: usize) -> bool {
        self.data = match self.data.get(offset..) {
            Some(data) => data,
            None => return false,
        };
        true
    }

    fn get_offset(&self) -> u8 {
        self.data.get(CHAR_WIDTH).cloned().unwrap()
    }

    pub fn jump_to_next(&mut self) -> bool {
        let offset = self.get_offset();
        if offset == 0 {
            false
        } else if self.jump_by(offset as usize) {
            true
        } else {
            panic!("There is an internal error in the current font data!");
        }
    }

    pub fn into_glyph_reader(self) -> GlyphReader {
        GlyphReader::new(self.data.get(CHAR_WIDTH + 1..).unwrap(), self.font)
    }
}

const U8G2_FONT_DATA_STRUCT_SIZE: usize = 23;

impl<'a> GlyphSearcher<'a, 1> {
    pub fn new(font: &'a FontReader) -> Self {
        Self {
            data: &font.data[U8G2_FONT_DATA_STRUCT_SIZE..],
            font,
        }
    }

    pub fn get_ch(&self) -> u8 {
        self.data.first().cloned().unwrap()
    }

    pub fn into_unicode_mode(
        mut self,
        offset: u16,
    ) -> (GlyphSearcher<'a, 2>, UnicodeJumptableReader) {
        if !self.jump_by(offset as usize) {
            panic!("Internal error in glyph data!");
        }

        (
            GlyphSearcher {
                data: self.data,
                font: self.font,
            },
            UnicodeJumptableReader::new(self.data),
        )
    }
}

impl<'a> GlyphSearcher<'a, 2> {
    pub fn get_ch(&self) -> u16 {
        u16::from_be_bytes([
            self.data.first().cloned().unwrap(),
            self.data.get(1).cloned().unwrap(),
        ])
    }
}
