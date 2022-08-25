use embedded_graphics_core::prelude::Point;

use crate::{
    draw_builder::common::compute_glyph_dimensions, font_reader::FontReader,
    types::RenderedDimensions, LookupError,
};

use super::{Content, LineDimensionsIterator};

pub struct GlyphContent(pub char);

impl Content for GlyphContent {
    fn for_each_char<F, E>(&self, mut func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        func(self.0)
    }

    fn for_each_char_infallible<F>(&self, mut func: F)
    where
        F: FnMut(char),
    {
        func(self.0)
    }

    fn get_newline_count(&self) -> u32 {
        0
    }

    type LineDimensionsIter = GlyphLineDimensionsIterator;

    fn line_dimensions_iterator(&self) -> GlyphLineDimensionsIterator {
        GlyphLineDimensionsIterator { ch: self.0 }
    }
}

pub struct GlyphLineDimensionsIterator {
    ch: char,
}

impl LineDimensionsIterator for GlyphLineDimensionsIterator {
    fn next(&mut self, font: &FontReader) -> Result<RenderedDimensions, LookupError> {
        compute_glyph_dimensions(self.ch, Point::new(0, 0), font)
    }
}
