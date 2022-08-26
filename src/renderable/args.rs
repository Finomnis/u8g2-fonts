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

    type LineDimensionsIter = ArgsLineDimensionsIterator;

    fn line_dimensions_iterator(&self) -> ArgsLineDimensionsIterator {
        ArgsLineDimensionsIterator {}
    }
}

pub struct ArgsLineDimensionsIterator {}
impl LineDimensionsIterator for ArgsLineDimensionsIterator {
    fn next(
        &mut self,
        _font: &crate::font_reader::FontReader,
    ) -> Result<RenderedDimensions, LookupError> {
        todo!()
    }
}
