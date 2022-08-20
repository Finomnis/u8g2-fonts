use embedded_graphics_core::{
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
    Pixel,
};

use crate::{font_reader::FontReader, glyph_reader::GlyphReader, types::VerticalPosition, Error};

pub struct GlyphRenderer<'a> {
    glyph: GlyphReader,
    font: &'a FontReader,
}

impl<'a> GlyphRenderer<'a> {
    pub fn new(glyph: &GlyphReader, font: &'a FontReader) -> Self {
        Self {
            glyph: glyph.clone(),
            font,
        }
    }

    pub fn get_glyph_bounding_box(
        &self,
        position: Point,
        vertical_pos: VerticalPosition,
    ) -> Rectangle {
        let mut topleft = self.glyph.topleft(&position);

        // Taken directly from U8g2 code
        let offset = match vertical_pos {
            VerticalPosition::Baseline => 0,
            VerticalPosition::Top => self.font.ascent as i32 + 1,
            VerticalPosition::Center => {
                (self.font.ascent as i32 - self.font.descent as i32 + 1) / 2
                    + self.font.descent as i32
            }
            VerticalPosition::Bottom => self.font.descent as i32,
        };

        topleft.y += offset;

        Rectangle::new(topleft, self.glyph.size())
    }

    pub fn render_as_box_fill<Display>(
        mut self,
        position: Point,
        vertical_pos: VerticalPosition,
        display: &mut Display,
        foreground_color: Display::Color,
        background_color: Display::Color,
    ) -> Result<(), Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let glyph_bounding_box = self.get_glyph_bounding_box(position, vertical_pos);

        let color_iter = {
            let mut num_zeros = self.glyph.read_runlength_0()?;
            let mut num_ones = self.glyph.read_runlength_1()?;
            let mut num_zeros_leftover = num_zeros;
            let mut num_ones_leftover = num_ones;
            move || -> Option<Display::Color> {
                if num_zeros_leftover == 0 && num_ones_leftover == 0 {
                    let repeat = self.glyph.read_unsigned::<Display::Error>(1).unwrap() != 0;
                    if !repeat {
                        num_zeros = self.glyph.read_runlength_0::<Display::Error>().unwrap();
                        num_ones = self.glyph.read_runlength_1::<Display::Error>().unwrap();
                    }
                    num_zeros_leftover = num_zeros;
                    num_ones_leftover = num_ones;
                }

                let color = if num_zeros_leftover > 0 {
                    num_zeros_leftover -= 1;
                    background_color
                } else {
                    num_ones_leftover -= 1;
                    foreground_color
                };

                Some(color)
            }
        };

        display
            .fill_contiguous(&glyph_bounding_box, core::iter::from_fn(color_iter))
            .map_err(Error::DisplayError)
    }

    pub fn render_transparent<Display>(
        mut self,
        position: Point,
        vertical_pos: VerticalPosition,
        display: &mut Display,
        foreground_color: Display::Color,
    ) -> Result<(), Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let glyph_bounding_box = self.get_glyph_bounding_box(position, vertical_pos);
        let width = glyph_bounding_box.size.width as i32;
        let height = glyph_bounding_box.size.height as i32;

        let pixel_iter = {
            let mut num_zeros = self.glyph.read_runlength_0()?;
            let mut num_ones = self.glyph.read_runlength_1()?;
            let mut num_ones_leftover = num_ones;

            let mut x = num_zeros as i32;
            let mut y = 0i32;

            while x >= width {
                x -= width;
                y += 1;
            }

            move || -> Option<Pixel<Display::Color>> {
                if y >= height {
                    return None;
                }

                while num_ones_leftover == 0 {
                    let repeat = self.glyph.read_unsigned::<Display::Error>(1).unwrap() != 0;
                    if !repeat {
                        num_zeros = self.glyph.read_runlength_0::<Display::Error>().unwrap();
                        num_ones = self.glyph.read_runlength_1::<Display::Error>().unwrap();
                    }
                    x += num_zeros as i32;
                    while x >= width {
                        x -= width;
                        y += 1;
                        if y >= height {
                            return None;
                        }
                    }
                    num_ones_leftover = num_ones;
                }

                let pixel = Pixel(
                    glyph_bounding_box.top_left + Point::new(x, y),
                    foreground_color,
                );
                x += 1;
                if x >= width {
                    x -= width;
                    y += 1;
                }
                num_ones_leftover -= 1;

                Some(pixel)
            }
        };

        display
            .draw_iter(core::iter::from_fn(pixel_iter))
            .map_err(Error::DisplayError)
    }
}
