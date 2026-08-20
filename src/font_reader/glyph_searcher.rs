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

pub const U8G2_FONT_DATA_STRUCT_SIZE: usize = 23;

/// An entry of the single byte encoding jump chain.
pub struct SingleByteEntry {
    pub encoding: u8,
    /// The distance to the next entry. A distance of zero terminates the chain.
    pub jump_distance: u8,
}

/// Reads the entry of the single byte encoding jump chain at `pos`.
///
/// `pos` is an index into `data`, not an offset into the glyph data.
///
/// Returns [`None`] if `data` is too short to contain the entry.
pub const fn read_single_byte_entry(data: &[u8], pos: usize) -> Option<SingleByteEntry> {
    if pos + 1 >= data.len() {
        return None;
    }

    Some(SingleByteEntry {
        encoding: data[pos],
        jump_distance: data[pos + 1],
    })
}

/// An entry of the unicode jump chain.
pub struct UnicodeEntry {
    /// An encoding of zero terminates the chain.
    pub encoding: u16,
    /// The distance to the next entry. A distance of zero terminates the chain.
    pub jump_distance: u8,
}

/// Reads the entry of the unicode jump chain at `pos`.
///
/// `pos` is an index into `data`, not an offset into the glyph data.
///
/// Returns [`None`] if `data` is too short to contain the entry. The chain
/// terminator consists of the encoding alone, so a missing jump distance is
/// reported as zero.
pub const fn read_unicode_entry(data: &[u8], pos: usize) -> Option<UnicodeEntry> {
    if pos + 1 >= data.len() {
        return None;
    }

    let jump_distance = if pos + 2 < data.len() {
        data[pos + 2]
    } else {
        0
    };

    Some(UnicodeEntry {
        encoding: u16::from_be_bytes([data[pos], data[pos + 1]]),
        jump_distance,
    })
}

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
        let entry = match read_single_byte_entry(data, U8G2_FONT_DATA_STRUCT_SIZE + offset) {
            Some(entry) => entry,
            None => return None,
        };

        // The terminator of the chain is not a glyph, so it is checked first.
        if entry.jump_distance == 0 {
            return None;
        }
        if entry.encoding == encoding {
            return Some(offset);
        }

        offset += entry.jump_distance as usize;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_byte_entries_need_an_encoding_and_a_jump_distance() {
        assert!(read_single_byte_entry(&[0x41], 0).is_none());

        let entry = read_single_byte_entry(&[0xff, 0x41, 0x07], 1).unwrap();
        assert_eq!(entry.encoding, 0x41);
        assert_eq!(entry.jump_distance, 0x07);
    }

    #[test]
    fn a_jump_chain_that_leaves_the_data_does_not_find_a_glyph() {
        let mut data = [0; U8G2_FONT_DATA_STRUCT_SIZE + 3];
        data[U8G2_FONT_DATA_STRUCT_SIZE] = b'A';
        // The jump distance points behind the end of the data.
        data[U8G2_FONT_DATA_STRUCT_SIZE + 1] = 4;

        assert!(find_glyph_offset(&data, 0, 0, b'B').is_none());
    }

    #[test]
    fn unicode_entries_need_an_encoding_and_may_omit_the_jump_distance() {
        assert!(read_unicode_entry(&[0x04], 0).is_none());

        let terminator = read_unicode_entry(&[0x00, 0x00], 0).unwrap();
        assert_eq!(terminator.encoding, 0x0000);
        assert_eq!(terminator.jump_distance, 0);

        let entry = read_unicode_entry(&[0x04, 0x10, 0x09], 0).unwrap();
        assert_eq!(entry.encoding, 0x0410);
        assert_eq!(entry.jump_distance, 0x09);
    }
}
