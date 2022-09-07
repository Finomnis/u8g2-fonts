use crate::{
    font_reader::FontReader, types::VerticalPosition, utils::HorizontalRenderedDimensions,
    LookupError,
};

mod args;
mod character;
mod text;
pub mod vertical_offset;

pub trait LineDimensionsIterator {
    fn next(&mut self, font: &FontReader) -> Result<HorizontalRenderedDimensions, LookupError>;
}

/// The datatypes that can be rendered by [`FontRenderer`](crate::FontRenderer).
pub trait Content {
    #[doc(hidden)]
    type LineDimensionsIter: LineDimensionsIterator;

    #[doc(hidden)]
    fn compute_vertical_offset(&self, font: &FontReader, vertical_pos: VerticalPosition) -> i32 {
        vertical_offset::compute_vertical_offset_from_dynamic_newlines(font, vertical_pos, || {
            self.get_newline_count().try_into().unwrap()
        })
    }

    #[doc(hidden)]
    fn get_newline_count(&self) -> u32 {
        let mut count = 0;
        self.for_each_char_infallible(|char| {
            if char == '\n' {
                count += 1;
            }
        });
        count
    }

    #[doc(hidden)]
    fn for_each_char<F, E>(&self, func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>;

    #[doc(hidden)]
    fn for_each_char_infallible<F>(&self, func: F)
    where
        F: FnMut(char);

    #[doc(hidden)]
    fn line_dimensions_iterator(&self) -> Self::LineDimensionsIter;
}
