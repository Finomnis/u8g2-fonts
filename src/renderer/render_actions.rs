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
            // From experiments, it seems that alignment looks more symmetrical
            // if everything is shifted by one in respect to the anchor point
            1
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
            // From experiments, it seems that alignment looks more symmetrical
            // if everything is shifted by one in respect to the anchor point
            1 - line_dimensions.advance
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
        let dimensions = compute_glyph_dimensions(
            ch,
            Point::new(position_x + line_dimensions.advance, 0),
            font,
        )?;
        line_dimensions.add(dimensions.into());
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
    let glyph = font.retrieve_glyph_data(ch)?;

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
