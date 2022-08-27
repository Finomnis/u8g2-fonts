use core::ops::Range;

use embedded_graphics_core::prelude::Point;

use crate::{
    font_reader::FontReader,
    renderer::render_actions::compute_glyph_dimensions,
    types::RenderedDimensions,
    utils::{combine_bounding_boxes, FormatArgsReader, FormatArgsReaderInfallible},
    Content, LookupError,
};

use super::LineDimensionsIterator;

impl<'a> Content for core::fmt::Arguments<'a> {
    fn for_each_char<F, E>(&self, mut func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        FormatArgsReader::new(|ch| func(ch).map(|()| true)).process_args(*self)
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
    finished: bool,
}

impl<'a> ArgsLineDimensionsIterator<'a> {
    pub fn new(args: core::fmt::Arguments<'a>) -> Self {
        Self {
            args,
            buffer_range: 0..0,
            dimensions_buffer: [(); NUM_BUFFERED_LINES].map(|()| RenderedDimensions::empty()),
            next_line: 0,
            finished: false,
        }
    }

    pub fn regenerate_buffer(
        &mut self,
        range_start: usize,
        font: &FontReader,
    ) -> Result<(), LookupError> {
        let mut line_advance = 0;
        let mut line_bounding_box = None;
        let mut line_num: usize = 0;

        FormatArgsReader::new(|ch| -> Result<bool, LookupError> {
            if ch == '\n' {
                if let Some(array_pos) = line_num.checked_sub(range_start) {
                    if let Some(cell) = self.dimensions_buffer.get_mut(array_pos) {
                        // If we are in the correct range, set the value in the array
                        cell.advance.x = line_advance;
                        cell.bounding_box = line_bounding_box;
                    }
                }
                line_num += 1;
                line_advance = 0;
                line_bounding_box = None;

                if line_num >= range_start + self.dimensions_buffer.len() {
                    // break if we are past the desired range
                    return Ok(false);
                }
            } else {
                let dimensions = compute_glyph_dimensions(ch, Point::new(line_advance, 0), font)?;
                line_bounding_box =
                    combine_bounding_boxes(line_bounding_box, dimensions.bounding_box);
                line_advance += dimensions.advance.x;
            }

            Ok(true)
        })
        .process_args(self.args)?;

        // One last time, if format_args ran out and our last line didn't end with a newline
        if let Some(array_pos) = line_num.checked_sub(range_start) {
            if let Some(cell) = self.dimensions_buffer.get_mut(array_pos) {
                // If we are in the correct range, set the value in the array
                cell.advance.x = line_advance;
                cell.bounding_box = line_bounding_box;

                // We hit the end, store that so we don't continue in future
                self.finished = true;
                line_num += 1;
            }
        }

        self.buffer_range = range_start..line_num;
        assert!(self.buffer_range.len() <= self.dimensions_buffer.len());

        Ok(())
    }
}

impl LineDimensionsIterator for ArgsLineDimensionsIterator<'_> {
    fn next(
        &mut self,
        font: &crate::font_reader::FontReader,
    ) -> Result<RenderedDimensions, LookupError> {
        let next_line = self.next_line;
        self.next_line += 1;

        if !self.buffer_range.contains(&next_line) {
            if self.finished {
                return Ok(RenderedDimensions::empty());
            }

            self.regenerate_buffer(next_line, font)?;
            assert!(self.buffer_range.contains(&next_line));
        }

        Ok(self.dimensions_buffer[next_line - self.buffer_range.start].clone())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use core::fmt::Arguments;
    use std::vec::Vec;

    use embedded_graphics_core::{prelude::Size, primitives::Rectangle};

    use crate::fonts;

    use super::*;

    #[test]
    fn for_each_char_produces_correct_values() {
        let mut content = Vec::new();

        format_args!("{}", "abc")
            .for_each_char(|e| {
                content.push(e);
                Result::<(), &'static str>::Ok(())
            })
            .unwrap();

        assert_eq!(content, ['a', 'b', 'c']);
    }

    #[test]
    fn for_each_char_infallible_produces_correct_values() {
        let mut content = Vec::new();

        format_args!("{}", "abc").for_each_char_infallible(|e| {
            content.push(e);
        });

        assert_eq!(content, ['a', 'b', 'c']);
    }

    #[test]
    fn for_each_char_propagates_error() {
        let result = format_args!("{}", "abc").for_each_char(|_| Err("Failed!"));

        assert_eq!(result, Err("Failed!"));
    }

    #[test]
    fn get_newline_count_provides_correct_value() {
        assert_eq!(format_args!("{}", "a\nbc\n").get_newline_count(), 2);
        assert_eq!(format_args!("{}", "a\nbc").get_newline_count(), 1);
        assert_eq!(format_args!("{}", "").get_newline_count(), 0);
    }

    #[test]
    fn line_dimensions_iter_provides_correct_values() {
        // Nested function to deal with format_args!()'s weird lifetimes
        fn run_test(args: Arguments<'_>) {
            let font = FontReader::new::<fonts::u8g2_font_u8glib_4_tf>();
            let mut dims = args.line_dimensions_iterator();

            assert_eq!(
                dims.next(&font).unwrap(),
                RenderedDimensions {
                    advance: Point::new(4, 0),
                    bounding_box: Some(Rectangle::new(Point::new(0, -3), Size::new(3, 3)))
                }
            );
            assert_eq!(
                dims.next(&font).unwrap(),
                RenderedDimensions {
                    advance: Point::new(7, 0),
                    bounding_box: Some(Rectangle::new(Point::new(0, -4), Size::new(6, 4)))
                }
            );
            assert_eq!(dims.next(&font).unwrap(), RenderedDimensions::empty());
            assert_eq!(dims.next(&font).unwrap(), RenderedDimensions::empty());
        }

        run_test(format_args!("{}", "a\nbc\n"));
    }

    #[test]
    fn line_dimensions_iter_errors_on_glyph_not_found() {
        // Nested function to deal with format_args!()'s weird lifetimes
        fn run_test(args: Arguments<'_>) {
            let font = FontReader::new::<fonts::u8g2_font_u8glib_4_tf>();
            let mut dims = args.line_dimensions_iterator();

            assert!(matches!(
                dims.next(&font),
                Err(LookupError::GlyphNotFound('☃'))
            ));
        }

        run_test(format_args!("{}", "☃"));
    }
}
