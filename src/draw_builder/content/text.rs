use embedded_graphics_core::prelude::Point;

use crate::{
    draw_builder::common::compute_line_dimensions, font_reader::FontReader,
    types::RenderedDimensions, LookupError,
};

use super::{Content, LineDimensionsIterator};

pub struct TextContent<'a>(pub &'a str);

impl<'a> Content for TextContent<'a> {
    fn for_each_char<F, E>(&self, mut func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        for ch in self.0.chars() {
            func(ch)?;
        }

        Ok(())
    }

    fn for_each_char_infallible<F>(&self, func: F)
    where
        F: FnMut(char),
    {
        self.0.chars().for_each(func);
    }

    type LineDimensionsIter = TextLineDimensionsIterator<'a>;

    fn line_dimensions_iterator(&self) -> TextLineDimensionsIterator<'a> {
        TextLineDimensionsIterator {
            data: self.0.lines(),
        }
    }
}

pub struct TextLineDimensionsIterator<'a> {
    data: core::str::Lines<'a>,
}

impl LineDimensionsIterator for TextLineDimensionsIterator<'_> {
    fn next(&mut self, font: &FontReader) -> Result<RenderedDimensions, LookupError> {
        let line = self.data.next().ok_or(LookupError::InternalError)?;
        compute_line_dimensions(line, Point::new(0, 0), font)
    }
}
