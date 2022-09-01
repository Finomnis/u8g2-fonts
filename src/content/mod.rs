use crate::{
    font_reader::FontReader,
    types::{RenderedDimensions, VerticalPosition},
    LookupError,
};

mod args;
mod character;
mod text;

pub trait LineDimensionsIterator {
    fn next(&mut self, font: &FontReader) -> Result<HorizontalRenderedDimensions, LookupError>;
}

/// Similar to [`RenderedDimensions`], but only in the horizontal axis.
/// Saves a lot of memory in [`args::ArgsLineDimensionsIterator`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HorizontalRenderedDimensions {
    /// The advance in x direction
    pub advance: i32,
    /// The width of the bounding box.
    /// 0 in the case of no bounding box.
    pub bounding_box_width: u32,
    /// The horizontal offset of the bounding box.
    /// 0 in the case of no bounding box.
    pub bounding_box_offset: i32,
}

impl HorizontalRenderedDimensions {
    pub fn empty() -> Self {
        Self {
            advance: 0,
            bounding_box_width: 0,
            bounding_box_offset: 0,
        }
    }
}

impl From<RenderedDimensions> for HorizontalRenderedDimensions {
    fn from(d: RenderedDimensions) -> Self {
        Self {
            advance: d.advance.x,
            bounding_box_width: d.bounding_box.map_or(0, |b| b.size.width),
            bounding_box_offset: d.bounding_box.map_or(0, |b| b.top_left.x),
        }
    }
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
    fn line_dimensions_iterator(&self) -> Self::LineDimensionsIter;
}
