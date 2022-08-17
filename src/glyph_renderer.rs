use embedded_graphics_core::{
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
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

    pub fn render_as_box_fill<Color, Display>(
        mut self,
        pos: Point,
        display: &mut Display,
        fg: Color,
        bg: Color,
    ) -> Result<(), Error<Display::Error>>
    where
        Color: Clone + core::fmt::Debug,
        Display: DrawTarget<Color = Color>,
        Display::Error: core::fmt::Debug,
    {
        let topleft = self.glyph.topleft(&pos);
        let size = self.glyph.size();

        let pixel_iter = {
            let mut num_zeros = self.glyph.read_runlength_0()?;
            let mut num_ones = self.glyph.read_runlength_1()?;
            let mut num_zeros_leftover = num_zeros;
            let mut num_ones_leftover = num_ones;
            move || -> Option<Color> {
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
                    bg.clone()
                } else {
                    num_ones_leftover -= 1;
                    fg.clone()
                };

                Some(color)
            }
        };

        display
            .fill_contiguous(
                &Rectangle::new(topleft, size),
                std::iter::from_fn(pixel_iter),
            )
            .map_err(Error::DisplayError)
    }
}
