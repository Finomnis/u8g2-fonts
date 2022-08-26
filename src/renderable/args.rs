use core::ops::Range;

use embedded_graphics_core::prelude::Point;

use crate::{
    types::RenderedDimensions,
    utils::{FormatArgsReader, FormatArgsReaderInfallible},
    LookupError, Renderable,
};

use super::LineDimensionsIterator;

impl<'a> Renderable for core::fmt::Arguments<'a> {
    fn for_each_char<F, E>(&self, func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        FormatArgsReader::new(func).process_args(*self)
    }

    fn for_each_char_infallible<F>(&self, func: F)
    where
        F: FnMut(char),
    {
        FormatArgsReaderInfallible::new(func).process_args(*self)
    }

    type LineDimensionsIter = ArgsLineDimensionsIterator<'a>;

    fn line_dimensions_iterator(&self) -> ArgsLineDimensionsIterator<'a> {
        ArgsLineDimensionsIterator::new(*self)
    }
}

const NUM_BUFFERED_LINES: usize = 20;

pub struct ArgsLineDimensionsIterator<'a> {
    args: core::fmt::Arguments<'a>,
    buffer_range: Range<usize>,
    dimensions_buffer: [RenderedDimensions; NUM_BUFFERED_LINES],
    next_line: usize,
}

impl<'a> ArgsLineDimensionsIterator<'a> {
    pub fn new(args: core::fmt::Arguments<'a>) -> Self {
        Self {
            args,
            buffer_range: 0..0,
            dimensions_buffer: [(); NUM_BUFFERED_LINES].map(|()| RenderedDimensions {
                advance: Point::new(0, 0),
                bounding_box: None,
            }),
            next_line: 0,
        }
    }
}

impl LineDimensionsIterator for ArgsLineDimensionsIterator<'_> {
    fn next(
        &mut self,
        _font: &crate::font_reader::FontReader,
    ) -> Result<RenderedDimensions, LookupError> {
        // TODO: implement
        Ok(RenderedDimensions {
            advance: Point::new(0, 0),
            bounding_box: None,
        })
    }
}
