mod args;
mod glyph;
mod text;

pub use args::ArgsContent;
pub use glyph::GlyphContent;
pub use text::TextContent;

use crate::{font_reader::FontReader, types::VerticalPosition};

pub trait Content {
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

    fn get_newline_count(&self) -> u32 {
        let mut count = 0;
        self.for_each_char_infallible(|char| {
            if char == '\n' {
                count += 1;
            }
        });
        count
    }

    fn for_each_char<F, E>(&self, func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>;

    fn for_each_char_infallible<F>(&self, func: F)
    where
        F: FnMut(char);
}
