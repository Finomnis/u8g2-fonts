use crate::{font_reader::FontReader, types::VerticalPosition};

pub const fn compute_vertical_offset_from_static_newlines(
    font: &FontReader,
    vertical_pos: VerticalPosition,
    newline_count: i32,
) -> i32 {
    assert!(font.line_height < i32::MAX as u32);
    let newline_advance = font.line_height as i32;
    let ascent = font.ascent as i32;
    let descent = font.descent as i32;

    match vertical_pos {
        VerticalPosition::Baseline => 0,
        VerticalPosition::Top => ascent + 1,
        VerticalPosition::Center => {
            let total_newline_advance = newline_count * newline_advance;
            (total_newline_advance + ascent - descent + 1) / 2 + descent - total_newline_advance
        }
        VerticalPosition::Bottom => descent - newline_count * newline_advance,
    }
}

// This one is faster, because it avoids computing the newline count if it isn't needed for the computation.
// It isn't const, however. That's why both exist.
pub fn compute_vertical_offset_from_dynamic_newlines(
    font: &FontReader,
    vertical_pos: VerticalPosition,
    newline_count: impl FnOnce() -> i32,
) -> i32 {
    match vertical_pos {
        VerticalPosition::Baseline | VerticalPosition::Top => {
            compute_vertical_offset_from_static_newlines(font, vertical_pos, 0)
        }
        VerticalPosition::Center | VerticalPosition::Bottom => {
            compute_vertical_offset_from_static_newlines(font, vertical_pos, newline_count())
        }
    }
}
