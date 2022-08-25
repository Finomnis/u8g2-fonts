use core::fmt::Arguments;

use crate::{
    types::RenderedDimensions,
    utils::{FormatArgsReader, FormatArgsReaderInfallible},
    LookupError,
};

use super::{Content, LineDimensionsIterator};

pub struct ArgsContent<'a>(pub Arguments<'a>);

impl<'a> Content for ArgsContent<'a> {
    fn for_each_char<F, E>(&self, func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        FormatArgsReader::new(func).process_args(self.0)
    }

    fn for_each_char_infallible<F>(&self, func: F)
    where
        F: FnMut(char),
    {
        FormatArgsReaderInfallible::new(func).process_args(self.0)
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
