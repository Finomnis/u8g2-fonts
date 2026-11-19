use crate::{
    font_reader::{
        cursor::Cursor, glyph_reader::GlyphReader,
        unicode_jumptable_reader::UnicodeJumptableReader, FontReader,
    },
    FontData, FontHeader, U8G2_FONT_HEADER_SIZE,
};

pub struct GlyphSearcher<'a, const CHAR_WIDTH: usize, D> {
    cursor: Cursor<D>,
    header: &'a FontHeader,
}

impl<'a, const CHAR_WIDTH: usize, D: FontData> GlyphSearcher<'a, CHAR_WIDTH, D> {
    pub fn jump_by(&mut self, offset: usize) {
        self.cursor.skip(offset);
    }

    fn get_offset(&self) -> u8 {
        self.cursor.peek(CHAR_WIDTH).unwrap()
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

    pub fn into_glyph_reader(self) -> GlyphReader<D> {
        let mut cursor = self.cursor;
        cursor.skip(CHAR_WIDTH + 1);
        GlyphReader::new(cursor, self.header)
    }
}

impl<'a, D: FontData> GlyphSearcher<'a, 1, D> {
    pub fn new(font: &'a FontReader<D>) -> Self {
        Self {
            cursor: Cursor::new(font.data.0, U8G2_FONT_HEADER_SIZE),
            header: &font.header,
        }
    }

    pub fn get_ch(&self) -> u8 {
        self.cursor.peek(0).unwrap()
    }

    pub fn into_unicode_mode(
        mut self,
        offset: u16,
    ) -> (GlyphSearcher<'a, 2, D>, UnicodeJumptableReader<D>) {
        self.jump_by(offset.into());

        (
            GlyphSearcher {
                cursor: self.cursor,
                header: self.header,
            },
            UnicodeJumptableReader::new(self.cursor),
        )
    }
}

impl<D: FontData> GlyphSearcher<'_, 2, D> {
    pub fn get_ch(&self) -> u16 {
        u16::from_be_bytes([self.cursor.peek(0).unwrap(), self.cursor.peek(1).unwrap()])
    }
}
