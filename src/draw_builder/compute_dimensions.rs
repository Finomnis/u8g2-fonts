use embedded_graphics_core::{prelude::Point, primitives::Rectangle};

use crate::{
    types::{HorizontalAlignment, RenderedDimensions},
    utils::combine_bounding_boxes,
    DrawBuilder, LookupError,
};

use super::{
    common::{compute_glyph_dimensions, compute_horizontal_offset},
    content::Content,
};

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
    args: &DrawBuilder<'_, T, C, HorizontalAlignment>,
) -> Result<Option<Rectangle>, LookupError>
where
    T: Content,
{
    let mut position = args.position;
    let font = args.font;
    let horizontal_align = args.horizontal_align;

    position.y += args
        .content
        .compute_vertical_offset(font, args.vertical_pos);

    let mut bounding_box = None;

    let mut line_advance = 0;
    let mut line_bounding_box = None;
    args.content
        .for_each_char(|ch| -> Result<(), LookupError> {
            if ch == '\n' {
                let horizontal_offset = compute_horizontal_offset(
                    horizontal_align,
                    RenderedDimensions {
                        advance: Point::new(line_advance, 0),
                        bounding_box: line_bounding_box,
                    },
                );

                // 'render' by moving the already known bounding box to the correct position
                if let Some(mut line_bounding_box) = line_bounding_box {
                    line_bounding_box.top_left.x += horizontal_offset;
                    line_bounding_box.top_left += position;
                    bounding_box = combine_bounding_boxes(bounding_box, Some(line_bounding_box));
                }

                line_advance = 0;
                line_bounding_box = None;
                position.y += font.font_bounding_box_height as i32 + 1;
            } else {
                let dimensions = compute_glyph_dimensions(ch, Point::new(line_advance, 0), font)?;
                line_bounding_box =
                    combine_bounding_boxes(line_bounding_box, dimensions.bounding_box);
                line_advance += dimensions.advance.x;
            }

            Ok(())
        })?;

    // One last pass, if the string didn't end with a newline
    let horizontal_offset = compute_horizontal_offset(
        horizontal_align,
        RenderedDimensions {
            advance: Point::new(line_advance, 0),
            bounding_box: line_bounding_box,
        },
    );

    if let Some(mut line_bounding_box) = line_bounding_box {
        line_bounding_box.top_left.x += horizontal_offset;
        line_bounding_box.top_left += position;
        bounding_box = combine_bounding_boxes(bounding_box, Some(line_bounding_box));
    }

    Ok(bounding_box)
}
