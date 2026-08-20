use crate::{
    font_reader::{
        glyph_searcher::{read_single_byte_entry, read_unicode_entry, U8G2_FONT_DATA_STRUCT_SIZE},
        FontReader,
    },
    utils::DebugIgnore,
};

/// The region of the font data that the iterator is currently walking.
#[derive(Clone, Debug)]
enum Region {
    /// Walking the single byte encoding chain, at the given index into the data.
    SingleByte(usize),
    /// Walking the unicode chain, at the given index into the data.
    Unicode(usize),
    /// Both chains have been walked to their end.
    Finished,
}

/// An iterator over the characters that a font contains.
///
/// Yields the characters in the order in which they are stored in the font,
/// which is ascending by code point.
///
/// Created by [`FontRenderer::glyphs()`](crate::FontRenderer::glyphs).
#[derive(Clone, Debug)]
pub struct Glyphs {
    data: DebugIgnore<&'static [u8]>,
    unicode_region: usize,
    region: Region,
}

impl Glyphs {
    pub(crate) fn new(font: &FontReader) -> Self {
        Self {
            data: DebugIgnore(font.data.0),
            unicode_region: U8G2_FONT_DATA_STRUCT_SIZE + usize::from(font.array_offset_0x0100),
            region: Region::SingleByte(U8G2_FONT_DATA_STRUCT_SIZE),
        }
    }

    /// Computes the position of the first entry of the unicode chain.
    ///
    /// The unicode region starts with the unicode jump table, whose first entry
    /// contains the distance from the region to the first glyph.
    fn first_unicode_entry(&self) -> Option<usize> {
        let data = self
            .data
            .get(self.unicode_region..self.unicode_region + 2)?;
        let jump_distance = u16::from_be_bytes([data[0], data[1]]);

        Some(self.unicode_region + usize::from(jump_distance))
    }
}

impl Iterator for Glyphs {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            match self.region {
                Region::SingleByte(pos) => match read_single_byte_entry(&self.data, pos) {
                    Some(entry) if entry.jump_distance != 0 => {
                        self.region = Region::SingleByte(pos + usize::from(entry.jump_distance));
                        return Some(char::from(entry.encoding));
                    }
                    _ => {
                        self.region = match self.first_unicode_entry() {
                            Some(pos) => Region::Unicode(pos),
                            None => Region::Finished,
                        };
                    }
                },
                Region::Unicode(pos) => {
                    let entry = match read_unicode_entry(&self.data, pos) {
                        Some(entry) if entry.encoding != 0 => entry,
                        _ => {
                            self.region = Region::Finished;
                            continue;
                        }
                    };

                    self.region = if entry.jump_distance == 0 {
                        Region::Finished
                    } else {
                        Region::Unicode(pos + usize::from(entry.jump_distance))
                    };

                    if let Some(ch) = char::from_u32(u32::from(entry.encoding)) {
                        return Some(ch);
                    }
                }
                Region::Finished => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec::Vec;

    use super::*;

    /// Builds font data out of a header whose unicode region starts right behind the
    /// terminator of the single byte encoding chain, that terminator, and the unicode
    /// region passed as arguments.
    macro_rules! font_data {
        ($($byte:expr),* $(,)?) => {
            &[
                0, 0, 4, 4, 8, 8, 8, 8, 8, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 2, // Header
                0, 0, // Terminator of the single byte encoding chain
                $($byte,)*
            ]
        };
    }

    fn glyphs_of(data: &'static [u8]) -> Vec<char> {
        FontReader::new_from_raw_data(data).glyphs().collect()
    }

    #[test]
    fn a_font_ends_if_its_unicode_region_lies_behind_the_data() {
        assert!(glyphs_of(font_data![]).is_empty());
    }

    #[test]
    fn the_last_unicode_entry_may_omit_its_jump_distance() {
        // A jump table that leads to a single entry that has no jump distance.
        assert_eq!(
            glyphs_of(font_data![0, 4, 0xff, 0xff, 0x04, 0x10]),
            ['\u{410}']
        );
    }

    #[test]
    fn unicode_entries_that_are_no_characters_are_skipped() {
        // 0xd800 is a surrogate, so it is not a character.
        assert!(glyphs_of(font_data![0, 4, 0xff, 0xff, 0xd8, 0x00, 3, 0, 0]).is_empty());
    }
}
