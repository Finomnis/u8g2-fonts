use crate::{
    font_reader::{glyph_searcher::find_glyph_offset, FontReader},
    Font,
};

/// The first character covered by a [`GlyphIndex`].
pub const FIRST_CHAR: u8 = 0x20;

/// The last character covered by a [`GlyphIndex`].
pub const LAST_CHAR: u8 = 0x7e;

/// The number of characters covered by a [`GlyphIndex`].
pub const NUM_CHARS: usize = (LAST_CHAR - FIRST_CHAR) as usize + 1;

/// Marks a character that the font does not contain.
const ABSENT: u16 = u16::MAX;

/// The result of a [`GlyphIndex`] lookup.
pub enum IndexLookup {
    Found(usize),
    Absent,
    NotCovered,
}

/// A table that maps the printable ASCII characters to the offsets of their glyphs.
///
/// The offsets are relative to the start of the glyph data, identical to what
/// [`find_glyph_offset`] computes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphIndex {
    offsets: [u16; NUM_CHARS],
}

impl GlyphIndex {
    /// Builds the index by walking the jump chain of the font once per character.
    ///
    /// Returns [`None`] if one of the offsets does not fit into a [`u16`], in which
    /// case the caller has to fall back to searching the jump chain at runtime.
    pub const fn build(
        data: &[u8],
        array_offset_upper_a: u16,
        array_offset_lower_a: u16,
    ) -> Option<Self> {
        let mut offsets = [ABSENT; NUM_CHARS];

        let mut i = 0;
        while i < NUM_CHARS {
            let encoding = FIRST_CHAR + i as u8;

            if let Some(offset) =
                find_glyph_offset(data, array_offset_upper_a, array_offset_lower_a, encoding)
            {
                if offset >= ABSENT as usize {
                    return None;
                }
                offsets[i] = offset as u16;
            }

            i += 1;
        }

        Some(Self { offsets })
    }

    /// Builds the index of the font `F`.
    pub const fn build_for<F: Font>() -> Option<Self> {
        let font = FontReader::new::<F>();

        Self::build(
            F::DATA,
            font.array_offset_upper_a,
            font.array_offset_lower_a,
        )
    }

    /// Looks up the offset of the glyph of `encoding`.
    pub const fn lookup(&self, encoding: u8) -> IndexLookup {
        if encoding < FIRST_CHAR || encoding > LAST_CHAR {
            return IndexLookup::NotCovered;
        }

        match self.offsets[(encoding - FIRST_CHAR) as usize] {
            ABSENT => IndexLookup::Absent,
            offset => IndexLookup::Found(offset as usize),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::{font_reader::glyph_searcher, fonts};

    /// Runs `check` on a selection of fonts with different glyph ranges, bit widths
    /// and encodings. `check` receives the font name, a plain and an indexed reader.
    fn for_each_test_font(check: impl Fn(&str, &FontReader, &FontReader)) {
        macro_rules! check_font {
            ($font:ty) => {
                check(
                    stringify!($font),
                    &FontReader::new::<$font>(),
                    &FontReader::new_indexed::<$font>(),
                )
            };
        }

        check_font!(fonts::u8g2_font_ncenB14_tr);
        check_font!(fonts::u8g2_font_courB10_tn);
        check_font!(fonts::u8g2_font_lubBI08_tf);
        check_font!(fonts::u8g2_font_u8glib_4_tf);
        check_font!(fonts::u8g2_font_4x6_mr);
        check_font!(fonts::u8g2_font_10x20_me);
        check_font!(fonts::u8g2_font_osb41_tf);
        check_font!(fonts::u8g2_font_haxrcorp4089_t_cyrillic);
        check_font!(fonts::u8g2_font_unifont_t_symbols);
    }

    #[test]
    fn index_matches_linear_search() {
        for_each_test_font(|name, plain, indexed| {
            assert!(indexed.glyph_index.is_some(), "{name} has no glyph index");

            for encoding in FIRST_CHAR..=LAST_CHAR {
                let ch = encoding as char;
                let expected = glyph_searcher::find_glyph_offset(
                    &plain.data,
                    plain.array_offset_upper_a,
                    plain.array_offset_lower_a,
                    encoding,
                );

                assert_eq!(
                    indexed.find_glyph_offset(encoding),
                    expected,
                    "{name}: index disagrees for {ch:?}"
                );
                assert_eq!(
                    plain.find_glyph_offset(encoding),
                    expected,
                    "{name}: linear search disagrees for {ch:?}"
                );
            }
        });
    }

    #[test]
    fn index_reports_absent_glyphs() {
        // A number-only font does not contain any letters.
        let font = FontReader::new_indexed::<fonts::u8g2_font_courB10_tn>();

        for encoding in *b"aAz" {
            assert!(matches!(
                font.glyph_index.unwrap().lookup(encoding),
                IndexLookup::Absent
            ));
            assert_eq!(font.find_glyph_offset(encoding), None);
        }

        assert!(matches!(
            font.glyph_index.unwrap().lookup(b'0'),
            IndexLookup::Found(_)
        ));
    }

    #[test]
    fn characters_outside_of_the_index_are_searched_linearly() {
        let font = FontReader::new::<fonts::u8g2_font_lubBI08_tf>();
        let indexed = FontReader::new_indexed::<fonts::u8g2_font_lubBI08_tf>();

        for encoding in [0x00u8, 0x1f, 0x7f, 0xe4, 0xff] {
            assert!(matches!(
                indexed.glyph_index.unwrap().lookup(encoding),
                IndexLookup::NotCovered
            ));
            assert_eq!(
                indexed.find_glyph_offset(encoding),
                font.find_glyph_offset(encoding)
            );
        }
    }

    // Proves that the whole index is computed at compile time.
    const INDEXED_FONT: FontReader = FontReader::new_indexed::<fonts::u8g2_font_ncenB14_tr>();

    const SPACE_LOOKUP: IndexLookup = match INDEXED_FONT.glyph_index {
        Some(index) => index.lookup(b' '),
        None => IndexLookup::NotCovered,
    };

    const _: () = assert!(INDEXED_FONT.glyph_index.is_some());
    const _: () = assert!(matches!(SPACE_LOOKUP, IndexLookup::Found(_)));
}
