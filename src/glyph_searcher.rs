use core::marker::PhantomData;

use crate::{font_reader::FontReader, glyph_reader::GlyphReader, Error};

pub struct AsciiMode;
pub struct Utf8Mode;

#[derive(Debug)]
pub struct GlyphSearcher<MODE> {
    data: &'static [u8],
    _mode: PhantomData<MODE>,
}

impl<MODE> GlyphSearcher<MODE> {
    pub fn jump_by(&mut self, offset: u16) -> bool {
        self.data = match self.data.get(offset as usize..) {
            Some(data) => data,
            None => return false,
        };
        true
    }
}

const U8G2_FONT_DATA_STRUCT_SIZE: usize = 23;

impl GlyphSearcher<AsciiMode> {
    pub fn new(font: &FontReader) -> Self {
        Self {
            data: &font.data[U8G2_FONT_DATA_STRUCT_SIZE..],
            _mode: PhantomData,
        }
    }

    fn get_offset(&self) -> Result<u8, Error> {
        self.data.get(1).cloned().ok_or(Error::InternalError)
    }

    pub fn get_ch(&self) -> Result<u8, Error> {
        self.data.get(0).cloned().ok_or(Error::InternalError)
    }

    pub fn jump_to_next(&mut self) -> Result<bool, Error> {
        let offset = self.get_offset()?;
        if offset == 0 {
            Ok(false)
        } else if self.jump_by(offset as u16) {
            Ok(true)
        } else {
            Err(Error::InternalError)
        }
    }

    pub fn into_glyph_reader(self) -> Result<GlyphReader, Error> {
        let offset = self.get_offset()?;
        Ok(GlyphReader::new(
            self.data
                .get(2..offset as usize)
                .ok_or(Error::InternalError)?,
        ))
    }
}

impl GlyphSearcher<Utf8Mode> {}
