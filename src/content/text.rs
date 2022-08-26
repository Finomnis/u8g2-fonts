use embedded_graphics_core::prelude::Point;

use crate::{
    font_reader::FontReader, renderer::render_actions::compute_line_dimensions,
    types::RenderedDimensions, Content, LookupError,
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
    fn next(&mut self, font: &FontReader) -> Result<RenderedDimensions, LookupError> {
        let line = self.data.next().unwrap_or("");
        compute_line_dimensions(line, Point::new(0, 0), font)
    }
}
