use embedded_graphics_core::prelude::{DrawTarget, Point};

use crate::{
    font_reader::FontReader,
    types::{FontColor, HorizontalAlignment, RenderedDimensions},
    utils::HorizontalRenderedDimensions,
    Error, LookupError,
};

pub fn compute_horizontal_offset(
    horizontal_align: HorizontalAlignment,
    line_dimensions: HorizontalRenderedDimensions,
) -> i32 {
    match horizontal_align {
        HorizontalAlignment::Left => {
            // No shift, left alignment is identical to `render()`
            0
        }
        HorizontalAlignment::Center => {
            if line_dimensions.bounding_box_width == 0 {
                0
            } else {
                let width = line_dimensions.bounding_box_width;
                let left = line_dimensions.bounding_box_offset;

                -(width as i32 / 2 + left)
            }
        }
        HorizontalAlignment::Right => {
            // `- 1` because otherwise we would shift it one too far
            -(line_dimensions.advance - 1)
        }
    }
}

pub fn compute_glyph_dimensions(
    ch: char,
    position: Point,
    font: &FontReader,
) -> Result<RenderedDimensions, LookupError> {
    let glyph = match font.try_retrieve_glyph_data(ch)? {
        Some(g) => g,
        None => {
            return Ok(RenderedDimensions::empty());
        }
    };

    let advance = glyph.advance();
    let size = glyph.size();

    let bounding_box = if size.width > 0 && size.height > 0 {
        let renderer = glyph.create_renderer();
        Some(renderer.get_glyph_bounding_box(position))
    } else {
        None
    };

    Ok(RenderedDimensions {
        advance: Point::new(advance as i32, 0),
        bounding_box,
    })
}

pub fn compute_horizontal_glyph_dimensions(
    ch: char,
    position_x: i32,
    font: &FontReader,
) -> Result<HorizontalRenderedDimensions, LookupError> {
    let glyph = match font.try_retrieve_glyph_data(ch)? {
        Some(g) => g,
        None => {
            return Ok(HorizontalRenderedDimensions::empty());
        }
    };

    let advance = glyph.advance() as i32;
    let width = glyph.width() as u32;
    let left = glyph.left(position_x);

    Ok(HorizontalRenderedDimensions {
        advance,
        bounding_box_offset: left,
        bounding_box_width: width,
    })
}

pub fn compute_horizontal_line_dimensions(
    line: &str,
    position_x: i32,
    font: &FontReader,
) -> Result<HorizontalRenderedDimensions, LookupError> {
    let mut line_dimensions = HorizontalRenderedDimensions {
        advance: position_x,
        bounding_box_width: 0,
        bounding_box_offset: 0,
    };

    for ch in line.chars() {
        let dimensions =
            compute_horizontal_glyph_dimensions(ch, position_x + line_dimensions.advance, font)?;
        line_dimensions.add(dimensions);
    }

    line_dimensions.advance -= position_x;
    Ok(line_dimensions)
}

pub fn render_glyph<Display>(
    ch: char,
    position: Point,
    color: FontColor<Display::Color>,
    font: &FontReader,
    display: &mut Display,
) -> Result<RenderedDimensions, Error<Display::Error>>
where
    Display: DrawTarget,
{
    let glyph = match font.try_retrieve_glyph_data(ch)? {
        Some(g) => g,
        None => {
            return Ok(RenderedDimensions::empty());
        }
    };

    let advance = glyph.advance();
    let size = glyph.size();

    let bounding_box = if size.width > 0 && size.height > 0 {
        let renderer = glyph.create_renderer();
        Some(match color {
            FontColor::Transparent(color) => {
                renderer.render_transparent(position, display, color)?
            }
            FontColor::WithBackground { fg, bg } => {
                renderer.render_as_box_fill(position, display, fg, bg)?
            }
        })
    } else {
        None
    };

    Ok(RenderedDimensions {
        advance: Point::new(advance as i32, 0),
        bounding_box,
    })
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn HorizontalOffset_Left() {
        let mut a = HorizontalRenderedDimensions::empty();

        a.advance = 5;
        a.bounding_box_offset = 10;
        a.bounding_box_width = 3;

        let offset = compute_horizontal_offset(HorizontalAlignment::Left, a);

        assert_eq!(offset, 0);
    }

    #[test]
    fn HorizontalOffset_Center_Odd() {
        let mut a = HorizontalRenderedDimensions::empty();

        a.advance = 5;
        a.bounding_box_offset = 10;
        a.bounding_box_width = 3;

        let offset = compute_horizontal_offset(HorizontalAlignment::Center, a);

        assert_eq!(offset, -11);
    }

    #[test]
    fn HorizontalOffset_Center_Even() {
        let mut a = HorizontalRenderedDimensions::empty();

        a.advance = 5;
        a.bounding_box_offset = 10;
        a.bounding_box_width = 4;

        let offset = compute_horizontal_offset(HorizontalAlignment::Center, a);

        assert_eq!(offset, -12);
    }

    #[test]
    fn HorizontalOffset_Right() {
        let mut a = HorizontalRenderedDimensions::empty();

        a.advance = 5;
        a.bounding_box_offset = 10;
        a.bounding_box_width = 6;

        let offset = compute_horizontal_offset(HorizontalAlignment::Right, a);

        assert_eq!(offset, -4);
    }
}
