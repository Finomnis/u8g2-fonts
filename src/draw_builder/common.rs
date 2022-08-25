use embedded_graphics_core::{prelude::Point, primitives::Rectangle};

use crate::{
    font_reader::FontReader,
    types::{HorizontalAlignment, RenderedDimensions},
    utils::combine_bounding_boxes,
    LookupError,
};

pub fn compute_horizontal_offset(
    horizontal_align: HorizontalAlignment,
    line_dimensions: RenderedDimensions,
) -> i32 {
    match horizontal_align {
        HorizontalAlignment::Left => {
            // From experiments, it seems that alignment looks more symmetrical
            // if everything is shifted by one in respect to the anchor point
            1
        }
        HorizontalAlignment::Center => {
            if let Some(bounding_box) = line_dimensions.bounding_box {
                let width = bounding_box.size.width;
                let left = bounding_box.top_left.x;

                -(width as i32 / 2 + left)
            } else {
                0
            }
        }
        HorizontalAlignment::Right => {
            // From experiments, it seems that alignment looks more symmetrical
            // if everything is shifted by one in respect to the anchor point
            1 - line_dimensions.advance.x
        }
    }
}

pub fn compute_glyph_dimensions(
    ch: char,
    position: Point,
    font: &FontReader,
) -> Result<RenderedDimensions, LookupError> {
    let glyph = font.retrieve_glyph_data(ch)?;

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

pub fn compute_line_dimensions(
    line: &str,
    mut position: Point,
    font: &FontReader,
) -> Result<RenderedDimensions, LookupError> {
    let mut bounding_box: Option<Rectangle> = None;

    let x0 = position.x;

    for ch in line.chars() {
        let dimensions = compute_glyph_dimensions(ch, position, font)?;
        position.x += dimensions.advance.x;
        bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
    }

    Ok(RenderedDimensions {
        advance: Point::new(position.x - x0, 0),
        bounding_box,
    })
}
