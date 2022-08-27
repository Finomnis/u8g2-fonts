use embedded_graphics_core::prelude::Point;

use crate::{
    font_reader::FontReader, renderer::render_actions::compute_glyph_dimensions,
    types::RenderedDimensions, Content, LookupError,
};

use super::LineDimensionsIterator;

impl Content for char {
    fn for_each_char<F, E>(&self, mut func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        func(*self)
    }

    fn for_each_char_infallible<F>(&self, mut func: F)
    where
        F: FnMut(char),
    {
        func(*self)
    }

    fn get_newline_count(&self) -> u32 {
        0
    }

    type LineDimensionsIter = CharLineDimensionsIterator;

    fn line_dimensions_iterator(&self) -> CharLineDimensionsIterator {
        CharLineDimensionsIterator { ch: Some(*self) }
    }
}

pub struct CharLineDimensionsIterator {
    ch: Option<char>,
}

impl LineDimensionsIterator for CharLineDimensionsIterator {
    fn next(&mut self, font: &FontReader) -> Result<RenderedDimensions, LookupError> {
        self.ch.take().map_or_else(
            || Ok(RenderedDimensions::empty()),
            |ch| compute_glyph_dimensions(ch, Point::new(0, 0), font),
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec::Vec;

    use embedded_graphics_core::{prelude::Size, primitives::Rectangle};

    use crate::fonts;

    use super::*;

    #[test]
    fn for_each_char_produces_correct_values() {
        let mut content = Vec::new();

        'a'.for_each_char(|e| {
            content.push(e);
            Result::<(), &'static str>::Ok(())
        })
        .unwrap();

        assert_eq!(content, ['a']);
    }

    #[test]
    fn for_each_char_infallible_produces_correct_values() {
        let mut content = Vec::new();

        'a'.for_each_char_infallible(|e| {
            content.push(e);
        });

        assert_eq!(content, ['a']);
    }

    #[test]
    fn for_each_char_propagates_error() {
        let result = 'a'.for_each_char(|_| Err("Failed!"));

        assert_eq!(result, Err("Failed!"));
    }

    #[test]
    fn get_newline_count_provides_correct_value() {
        assert_eq!('a'.get_newline_count(), 0);
    }

    #[test]
    fn line_dimensions_iter_provides_correct_values() {
        let font = FontReader::new::<fonts::u8g2_font_u8glib_4_tf>();
        let ch = 'a';
        let mut dims = ch.line_dimensions_iterator();

        assert_eq!(
            dims.next(&font).unwrap(),
            RenderedDimensions {
                advance: Point::new(4, 0),
                bounding_box: Some(Rectangle::new(Point::new(0, -3), Size::new(3, 3)))
            }
        );
        assert_eq!(dims.next(&font).unwrap(), RenderedDimensions::empty());
    }

    #[test]
    fn line_dimensions_iter_errors_on_glyph_not_found() {
        let font = FontReader::new::<fonts::u8g2_font_u8glib_4_tf>();
        let ch = '☃';
        let mut dims = ch.line_dimensions_iterator();

        assert!(matches!(
            dims.next(&font),
            Err(LookupError::GlyphNotFound('☃'))
        ));
    }
}
