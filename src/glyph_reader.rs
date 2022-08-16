use core::marker::PhantomData;

use crate::{font_reader::FontReader, Error};

pub struct SEARCH_MODE;
pub struct READ_MODE;

pub struct GlyphReader<MODE> {
    data: &'static [u8],
    _mode: PhantomData<MODE>,
}

impl<MODE> GlyphReader<MODE> {}

const U8G2_FONT_DATA_STRUCT_SIZE: usize = 23;

impl GlyphReader<SEARCH_MODE> {
    pub fn new(font: &FontReader) -> Self {
        Self {
            data: &font.data[U8G2_FONT_DATA_STRUCT_SIZE..],
            _mode: PhantomData,
        }
    }

    pub fn jump_by(&mut self, offset: u16) -> bool {
        self.data = match self.data.get(offset as usize..) {
            Some(data) => data,
            None => return false,
        };
        true
    }

    pub fn get_ch(&self) -> Result<u8, Error> {
        self.data.get(0).cloned().ok_or(Error::INTERNAL_ERROR)
    }

    pub fn jump_to_next(&mut self) -> Result<bool, Error> {
        let offset = *self.data.get(1).ok_or(Error::INTERNAL_ERROR)?;
        if offset == 0 {
            Ok(false)
        } else if self.jump_by(offset as u16) {
            Ok(true)
        } else {
            Err(Error::INTERNAL_ERROR)
        }
    }
}

impl GlyphReader<READ_MODE> {}
