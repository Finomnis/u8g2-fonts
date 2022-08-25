use crate::{font_reader::FontReader, types::RenderedDimensions, LookupError};

pub trait LineDimensionsIterator {
    fn next(&mut self, font: &FontReader) -> Result<RenderedDimensions, LookupError>;
}
