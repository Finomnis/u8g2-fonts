use embedded_graphics_core::{prelude::Point, primitives::Rectangle};

use crate::{
    font_reader::FontReader,
    types::{HorizontalAlignment, RenderedDimensions},
    utils::combine_bounding_boxes,
    DrawBuilder, LookupError,
};

use super::content::Content;

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
    position: Point,
    font: &FontReader,
) -> Result<RenderedDimensions, LookupError> {
    let mut bounding_box: Option<Rectangle> = None;
    let mut advance = 0;

    for ch in line.chars() {
        let dimensions = compute_glyph_dimensions(ch, position, font)?;
        advance += dimensions.advance.x;
        bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
    }

    Ok(RenderedDimensions {
        advance: Point::new(advance, 0),
        bounding_box,
    })
}

pub fn compute_dimensions_unaligned<T, C>(
    args: &DrawBuilder<'_, T, C, ()>,
) -> Result<RenderedDimensions, LookupError>
where
    T: Content,
{
    let mut position = args.position;
    let font = args.font;

    let mut advance = Point::new(0, 0);

    let mut bounding_box = None;

    position.y += args
        .content
        .compute_vertical_offset(font, args.vertical_pos);

    args.content
        .for_each_char(|ch| -> Result<(), LookupError> {
            if ch == '\n' {
                advance.x = 0;
                advance.y += font.font_bounding_box_height as i32 + 1;
            } else {
                let dimensions = compute_glyph_dimensions(ch, position + advance, font)?;
                advance += dimensions.advance;
                bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
            }

            Ok(())
        })?;

    Ok(RenderedDimensions {
        advance,
        bounding_box,
    })
}

pub fn compute_dimensions_aligned<T, C>(
    _args: &DrawBuilder<'_, T, C, HorizontalAlignment>,
) -> Result<Option<Rectangle>, LookupError>
where
    T: Content,
{
    todo!()
}
