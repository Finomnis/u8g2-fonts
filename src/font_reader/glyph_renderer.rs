use embedded_graphics_core::{
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
    Pixel,
};

use crate::{font_reader::glyph_reader::GlyphReader, Error};

pub struct GlyphRenderer {
    glyph: GlyphReader,
}

impl GlyphRenderer {
    pub fn new(glyph: &GlyphReader) -> Self {
        Self {
            glyph: glyph.clone(),
        }
    }

    pub fn get_glyph_bounding_box(&self, position: Point) -> Rectangle {
        Rectangle::new(self.glyph.topleft(&position), self.glyph.size())
    }

    pub fn render_as_box_fill<Display>(
        mut self,
        position: Point,
        display: &mut Display,
        foreground_color: Display::Color,
        background_color: Display::Color,
    ) -> Result<Rectangle, Error<Display::Error>>
    where
        Display: DrawTarget,
    {
        let glyph_bounding_box = self.get_glyph_bounding_box(position);

        let color_iter = {
            let mut num_zeros = self.glyph.read_runlength_0();
            let mut num_ones = self.glyph.read_runlength_1();
            let mut num_zeros_leftover = num_zeros;
            let mut num_ones_leftover = num_ones;
            move || -> Option<Display::Color> {
                if num_zeros_leftover == 0 && num_ones_leftover == 0 {
                    let repeat = self.glyph.read_unsigned(1) != 0;
                    if !repeat {
                        num_zeros = self.glyph.read_runlength_0();
                        num_ones = self.glyph.read_runlength_1();
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
            .map_err(Error::DisplayError)?;

        Ok(glyph_bounding_box)
    }

    pub fn render_with_advance_fill<Display>(
        mut self,
        position: Point,
        display: &mut Display,
        foreground_color: Display::Color,
        background_color: Display::Color,
    ) -> Result<Rectangle, Error<Display::Error>>
    where
        Display: DrawTarget,
    {
        let offset_x = self.glyph.left(0) as i8;
        let glyph_width = self.glyph.width() as u32;
        let glyph_advance = self.glyph.advance();

        let left_padding = offset_x.max(0) as u32;
        let right_padding = (glyph_advance - offset_x - glyph_width as i8).max(0) as u32;

        let glyph_pos = self.glyph.topleft(&position);
        let advance_pos = Point::new(glyph_pos.x - left_padding as i32, glyph_pos.y);

        let glyph_size = self.glyph.size();
        let total_width = glyph_size.width + left_padding + right_padding;
        let total_size = Size::new(total_width, glyph_size.height);

        let advance_bounding_box = Rectangle::new(advance_pos, total_size);

        let color_iter = {
            let mut x: u32 = 0;

            let mut num_zeros = self.glyph.read_runlength_0();
            let mut num_ones = self.glyph.read_runlength_1();
            let mut num_zeros_leftover = num_zeros;
            let mut num_ones_leftover = num_ones;

            let total_width = self.glyph.advance() as u32;

            move || -> Option<Display::Color> {
                let is_padding = x < left_padding || x >= (glyph_width + left_padding);
                x = (x + 1) % total_width;

                if is_padding {
                    return Some(background_color);
                }

                if num_zeros_leftover == 0 && num_ones_leftover == 0 {
                    let repeat = self.glyph.read_unsigned(1) != 0;
                    if !repeat {
                        num_zeros = self.glyph.read_runlength_0();
                        num_ones = self.glyph.read_runlength_1();
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
            .fill_contiguous(&advance_bounding_box, core::iter::from_fn(color_iter))
            .map_err(Error::DisplayError)?;

        Ok(advance_bounding_box)
    }

    pub fn render_transparent<Display>(
        mut self,
        position: Point,
        display: &mut Display,
        foreground_color: Display::Color,
    ) -> Result<Rectangle, Error<Display::Error>>
    where
        Display: DrawTarget,
    {
        let glyph_bounding_box = self.get_glyph_bounding_box(position);
        let width = glyph_bounding_box.size.width as i32;
        let height = glyph_bounding_box.size.height as i32;

        let pixel_iter = {
            let mut num_zeros = self.glyph.read_runlength_0();
            let mut num_ones = self.glyph.read_runlength_1();
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
                    let repeat = self.glyph.read_unsigned(1) != 0;
                    if !repeat {
                        num_zeros = self.glyph.read_runlength_0();
                        num_ones = self.glyph.read_runlength_1();
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
            .map_err(Error::DisplayError)?;

        Ok(glyph_bounding_box)
    }
}
