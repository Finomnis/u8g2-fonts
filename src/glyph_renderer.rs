use embedded_graphics_core::{
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
    Pixel,
};

use crate::{glyph_reader::GlyphReader, Error};

pub struct GlyphRenderer {
    glyph: GlyphReader,
}

impl GlyphRenderer {
    pub fn new(glyph: &GlyphReader) -> Self {
        Self {
            glyph: glyph.clone(),
        }
    }

    pub fn render_as_box_fill<Display>(
        mut self,
        position: Point,
        display: &mut Display,
        foreground_color: Display::Color,
        background_color: Display::Color,
    ) -> Result<(), Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let topleft = self.glyph.topleft(&position);
        let size = self.glyph.size();

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
            .fill_contiguous(
                &Rectangle::new(topleft, size),
                core::iter::from_fn(color_iter),
            )
            .map_err(Error::DisplayError)
    }

    pub fn render_transparent<Display>(
        mut self,
        position: Point,
        display: &mut Display,
        foreground_color: Display::Color,
    ) -> Result<(), Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        let topleft = self.glyph.topleft(&position);
        let size = self.glyph.size();
        let width = size.width as i32;
        let height = size.height as i32;

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

                let pixel = Pixel(Point::new(topleft.x + x, topleft.y + y), foreground_color);
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