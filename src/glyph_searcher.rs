use crate::{font_reader::FontReader, glyph_reader::GlyphReader, Error};

#[derive(Debug)]
pub struct GlyphSearcher<const CHAR_WIDTH: usize, const JUMPTABLE_MODE: bool> {
    data: &'static [u8],
}

impl<const CHAR_WIDTH: usize> GlyphSearcher<CHAR_WIDTH, false> {
    pub fn jump_by(&mut self, offset: u16) -> bool {
        self.data = match self.data.get(offset as usize..) {
            Some(data) => data,
            None => return false,
        };
        true
    }

    fn get_offset<DisplayError>(&self) -> Result<u8, Error<DisplayError>> {
        self.data
            .get(CHAR_WIDTH)
            .cloned()
            .ok_or(Error::InternalError)
    }

    pub fn jump_to_next<DisplayError>(&mut self) -> Result<bool, Error<DisplayError>> {
        let offset = self.get_offset()?;
        if offset == 0 {
            Ok(false)
        } else if self.jump_by(offset as u16) {
            Ok(true)
        } else {
            Err(Error::InternalError)
        }
    }

    pub fn into_glyph_reader<DisplayError>(self) -> Result<GlyphReader, Error<DisplayError>> {
        Ok(GlyphReader::new(
            self.data
                .get(CHAR_WIDTH + 1..)
                .ok_or(Error::InternalError)?,
        ))
    }
}

const U8G2_FONT_DATA_STRUCT_SIZE: usize = 23;

impl GlyphSearcher<1, false> {
    pub fn new(font: &FontReader) -> Self {
        Self {
            data: &font.data[U8G2_FONT_DATA_STRUCT_SIZE..],
        }
    }

    pub fn get_ch<DisplayError>(&self) -> Result<u8, Error<DisplayError>> {
        self.data.get(0).cloned().ok_or(Error::InternalError)
    }

    pub fn into_unicode_mode<DisplayError>(
        mut self,
        offset: u16,
    ) -> Result<GlyphSearcher<2, true>, Error<DisplayError>> {
        if self.jump_by(offset) {
            Ok(GlyphSearcher { data: self.data })
        } else {
            Err(Error::InternalError)
        }
    }
}

impl GlyphSearcher<2, true> {}
