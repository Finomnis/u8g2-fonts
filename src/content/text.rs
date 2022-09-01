use crate::{
    font_reader::FontReader, renderer::render_actions::compute_horizontal_line_dimensions,
    utils::HorizontalRenderedDimensions, Content, LookupError,
};

use super::LineDimensionsIterator;

impl<'a> Content for &'a str {
    fn for_each_char<F, E>(&self, mut func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        for ch in self.chars() {
            func(ch)?;
        }

        Ok(())
    }

    fn for_each_char_infallible<F>(&self, func: F)
    where
        F: FnMut(char),
    {
        self.chars().for_each(func);
    }

    type LineDimensionsIter = TextLineDimensionsIterator<'a>;

    fn line_dimensions_iterator(&self) -> TextLineDimensionsIterator<'a> {
        TextLineDimensionsIterator { data: self.lines() }
    }
}

pub struct TextLineDimensionsIterator<'a> {
    data: core::str::Lines<'a>,
}

impl LineDimensionsIterator for TextLineDimensionsIterator<'_> {
    fn next(&mut self, font: &FontReader) -> Result<HorizontalRenderedDimensions, LookupError> {
        let line = self.data.next().unwrap_or("");
        compute_horizontal_line_dimensions(line, 0, font)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec::Vec;

    use crate::fonts;

    use super::*;

    #[test]
    fn for_each_char_produces_correct_values() {
        let mut content = Vec::new();

        "abc"
            .for_each_char(|e| {
                content.push(e);
                Result::<(), &'static str>::Ok(())
            })
            .unwrap();

        assert_eq!(content, ['a', 'b', 'c']);
    }

    #[test]
    fn for_each_char_infallible_produces_correct_values() {
        let mut content = Vec::new();

        "abc".for_each_char_infallible(|e| {
            content.push(e);
        });

        assert_eq!(content, ['a', 'b', 'c']);
    }

    #[test]
    fn for_each_char_propagates_error() {
        let result = "abc".for_each_char(|_| Err("Failed!"));

        assert_eq!(result, Err("Failed!"));
    }

    #[test]
    fn get_newline_count_provides_correct_value() {
        assert_eq!("a\nbc\n".get_newline_count(), 2);
        assert_eq!("a\nbc".get_newline_count(), 1);
        assert_eq!("".get_newline_count(), 0);
    }

    #[test]
    fn line_dimensions_iter_provides_correct_values() {
        let font = FontReader::new::<fonts::u8g2_font_u8glib_4_tf>();
        let text = "a\nbc\n";
        let mut dims = text.line_dimensions_iterator();

        assert_eq!(
            dims.next(&font).unwrap(),
            HorizontalRenderedDimensions {
                advance: 4,
                bounding_box_width: 3,
                bounding_box_offset: 0,
            }
        );
        assert_eq!(
            dims.next(&font).unwrap(),
            HorizontalRenderedDimensions {
                advance: 7,
                bounding_box_width: 6,
                bounding_box_offset: 0,
            }
        );
        assert_eq!(
            dims.next(&font).unwrap(),
            HorizontalRenderedDimensions::empty()
        );
        assert_eq!(
            dims.next(&font).unwrap(),
            HorizontalRenderedDimensions::empty()
        );
    }

    #[test]
    fn line_dimensions_iter_errors_on_glyph_not_found() {
        let font = FontReader::new::<fonts::u8g2_font_u8glib_4_tf>();
        let text = "a\n☃";
        let mut dims = text.line_dimensions_iterator();

        assert_eq!(
            dims.next(&font).unwrap(),
            HorizontalRenderedDimensions {
                advance: 4,
                bounding_box_width: 3,
                bounding_box_offset: 0,
            }
        );
        assert!(matches!(
            dims.next(&font),
            Err(LookupError::GlyphNotFound('☃'))
        ));
    }
}
