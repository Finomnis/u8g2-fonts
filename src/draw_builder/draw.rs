use embedded_graphics_core::{
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
};

use crate::{
    font_reader::FontReader,
    types::{HorizontalAlignment, RenderedDimensions},
    utils::combine_bounding_boxes,
    DrawBuilder, Error,
};

use super::{
    common::compute_horizontal_offset, content::Content,
    line_dimensions_iterator::LineDimensionsIterator, DrawColor,
};

pub fn draw_unaligned<T, Display>(
    args: &DrawBuilder<'_, T, DrawColor<Display::Color>, ()>,
    display: &mut Display,
) -> Result<RenderedDimensions, Error<Display::Error>>
where
    T: Content,
    Display: DrawTarget,
    Display::Error: core::fmt::Debug,
{
}

pub fn draw_aligned<T, Display>(
    args: &DrawBuilder<'_, T, DrawColor<Display::Color>, HorizontalAlignment>,
    display: &mut Display,
) -> Result<Option<Rectangle>, Error<Display::Error>>
where
    T: Content,
    Display: DrawTarget,
    Display::Error: core::fmt::Debug,
{
    // This function is a little more complicated.
    // To properly align horizontally, we need to iterate over every line twice.
    // This is really hard with format_args.
    // Therefore we introduce a line_dimensions_iterator that is almost no overhead for
    // glyphs/lines, but makes it possible to implement the format_args case.

    let mut position = args.position;
    let font = args.font;
    let horizontal_align = args.horizontal_align;

    position.y += args
        .content
        .compute_vertical_offset(font, args.vertical_pos);

    let mut bounding_box = None;

    let mut line_dimensions = args.content.line_dimensions_iterator();
    let mut advance = Point::new(
        compute_horizontal_offset(horizontal_align, line_dimensions.next(font)?),
        0,
    );

    args.content
        .for_each_char(|ch| -> Result<(), Error<Display::Error>> {
            if ch == '\n' {
                advance.x =
                    compute_horizontal_offset(horizontal_align, line_dimensions.next(font)?);
                advance.y += font.font_bounding_box_height as i32 + 1;
            } else {
                let dimensions = render_glyph(
                    ch,
                    position + advance,
                    args.color.fg,
                    args.color.bg,
                    font,
                    display,
                )?;
                advance += dimensions.advance;
                bounding_box = combine_bounding_boxes(bounding_box, dimensions.bounding_box);
            }

            Ok(())
        })?;

    Ok(bounding_box)
}
