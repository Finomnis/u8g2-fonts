use crate::{
    font_reader::FontReader,
    types::{RenderedDimensions, VerticalPosition},
    LookupError,
};

mod args;
mod character;
mod text;

pub trait LineDimensionsIterator {
    fn next(&mut self, font: &FontReader) -> Result<RenderedDimensions, LookupError>;
}

/// The datatypes that can be rendered by [`FontRenderer`](crate::FontRenderer).
pub trait Content {
    #[doc(hidden)]
    type LineDimensionsIter: LineDimensionsIterator;

    #[doc(hidden)]
    fn compute_vertical_offset(&self, font: &FontReader, vertical_pos: VerticalPosition) -> i32 {
        let newline_advance = font.font_bounding_box_height as i32 + 1;
        let ascent = font.ascent as i32;
        let descent = font.descent as i32;

        match vertical_pos {
            VerticalPosition::Baseline => 0,
            VerticalPosition::Top => ascent + 1,
            VerticalPosition::Center => {
                let total_newline_advance = self.get_newline_count() as i32 * newline_advance;
                (total_newline_advance + ascent - descent + 1) / 2 + descent - total_newline_advance
            }
            VerticalPosition::Bottom => descent - self.get_newline_count() as i32 * newline_advance,
        }
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
    fn line_dimensions_iterator<'a>(&self) -> Self::LineDimensionsIter;
}
