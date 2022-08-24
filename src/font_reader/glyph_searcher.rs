use crate::{
    font_reader::{
        glyph_reader::GlyphReader, unicode_jumptable_reader::UnicodeJumptableReader, FontReader,
    },
    LookupError,
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

    fn get_offset(&self) -> Result<u8, LookupError> {
        self.data
            .get(CHAR_WIDTH)
            .cloned()
            .ok_or(LookupError::InternalError)
    }

    pub fn jump_to_next(&mut self) -> Result<bool, LookupError> {
        let offset = self.get_offset()?;
        if offset == 0 {
            Ok(false)
        } else if self.jump_by(offset as usize) {
            Ok(true)
        } else {
            Err(LookupError::InternalError)
        }
    }

    pub fn into_glyph_reader(self) -> Result<GlyphReader, LookupError> {
        GlyphReader::new(
            self.data
                .get(CHAR_WIDTH + 1..)
                .ok_or(LookupError::InternalError)?,
            self.font,
        )
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

    pub fn get_ch(&self) -> Result<u8, LookupError> {
        self.data.first().cloned().ok_or(LookupError::InternalError)
    }

    pub fn into_unicode_mode(
        mut self,
        offset: u16,
    ) -> Result<(GlyphSearcher<'a, 2>, UnicodeJumptableReader), LookupError> {
        if self.jump_by(offset as usize) {
            Ok((
                GlyphSearcher {
                    data: self.data,
                    font: self.font,
                },
                UnicodeJumptableReader::new(self.data),
            ))
        } else {
            Err(LookupError::InternalError)
        }
    }
}

impl<'a> GlyphSearcher<'a, 2> {
    pub fn get_ch(&self) -> Result<u16, LookupError> {
        Ok(u16::from_be_bytes([
            self.data
                .first()
                .cloned()
                .ok_or(LookupError::InternalError)?,
            self.data
                .get(1)
                .cloned()
                .ok_or(LookupError::InternalError)?,
        ]))
    }
}
