use core::{mem, ops::Range};

use crate::{
    font_reader::FontReader,
    renderer::render_actions::compute_horizontal_glyph_dimensions,
    utils::{FormatArgsReader, FormatArgsReaderInfallible, HorizontalRenderedDimensions},
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

// Most strings will print only a single line.
// Having a buffer of 5 should be fine for most embedded systems.
// (5* sizeof(HorizontalRenderedDimensions)), which should be in the range of
// ~60 bytes
const NUM_BUFFERED_LINES: usize = 5;

pub struct ArgsLineDimensionsIterator<'a> {
    args: core::fmt::Arguments<'a>,
    buffer_range: Range<usize>,
    dimensions_buffer: [HorizontalRenderedDimensions; NUM_BUFFERED_LINES],
    next_line: usize,
    finished: bool,
}

impl<'a> ArgsLineDimensionsIterator<'a> {
    pub fn new(args: core::fmt::Arguments<'a>) -> Self {
        Self {
            args,
            buffer_range: 0..0,
            dimensions_buffer: [(); NUM_BUFFERED_LINES]
                .map(|()| HorizontalRenderedDimensions::empty()),
            next_line: 0,
            finished: false,
        }
    }

    pub fn regenerate_buffer(
        &mut self,
        range_start: usize,
        font: &FontReader,
    ) -> Result<(), LookupError> {
        let mut line_dimensions = HorizontalRenderedDimensions::empty();
        let mut line_num: usize = 0;

        FormatArgsReader::new(|ch| -> Result<bool, LookupError> {
            if ch == '\n' {
                let previous_line_dimensions =
                    mem::replace(&mut line_dimensions, HorizontalRenderedDimensions::empty());

                if let Some(array_pos) = line_num.checked_sub(range_start) {
                    if let Some(cell) = self.dimensions_buffer.get_mut(array_pos) {
                        // If we are in the correct range, set the value in the array
                        *cell = previous_line_dimensions;
                    }
                }

                line_num += 1;

                if line_num >= range_start + self.dimensions_buffer.len() {
                    // break if we are past the desired range
                    return Ok(false);
                }
            } else if line_num >= range_start {
                // Only compute dimensions if we are in a line that will be buffered
                let dimensions =
                    compute_horizontal_glyph_dimensions(ch, line_dimensions.advance, font)?;
                line_dimensions.add(dimensions);
            }

            Ok(true)
        })
        .process_args(self.args)?;

        // One last time, if format_args ran out and our last line didn't end with a newline
        if let Some(array_pos) = line_num.checked_sub(range_start) {
            if let Some(cell) = self.dimensions_buffer.get_mut(array_pos) {
                // If we are in the correct range, set the value in the array
                *cell = line_dimensions;

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
    ) -> Result<HorizontalRenderedDimensions, LookupError> {
        let next_line = self.next_line;
        self.next_line += 1;

        if !self.buffer_range.contains(&next_line) {
            if self.finished {
                return Ok(HorizontalRenderedDimensions::empty());
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
                HorizontalRenderedDimensions {
                    advance: 4,
                    bounding_box_width: 3,
                    bounding_box_offset: 0,
                }
            );
            assert_eq!(
                dims.next(&font).unwrap(),
                HorizontalRenderedDimensions {
                    advance: 7,
                    bounding_box_width: 6,
                    bounding_box_offset: 0,
                }
            );
            assert_eq!(
                dims.next(&font).unwrap(),
                HorizontalRenderedDimensions::empty()
            );
            assert_eq!(
                dims.next(&font).unwrap(),
                HorizontalRenderedDimensions::empty()
            );
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

    #[test]
    fn line_dimensions_iter_creates_empty_array_when_out_of_range() {
        // Nested function to deal with format_args!()'s weird lifetimes
        fn run_test(args: Arguments<'_>) {
            let font = FontReader::new::<fonts::u8g2_font_u8glib_4_tf>();
            let mut dims = args.line_dimensions_iterator();

            dims.regenerate_buffer(1000, &font).unwrap();
            assert!(dims.buffer_range.is_empty());
        }

        run_test(format_args!("{}", "a"));
    }
}
