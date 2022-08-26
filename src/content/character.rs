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
        CharLineDimensionsIterator { ch: *self }
    }
}

pub struct CharLineDimensionsIterator {
    ch: char,
}

impl LineDimensionsIterator for CharLineDimensionsIterator {
    fn next(&mut self, font: &FontReader) -> Result<RenderedDimensions, LookupError> {
        compute_glyph_dimensions(self.ch, Point::new(0, 0), font)
    }
}
