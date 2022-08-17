use embedded_graphics_core::{
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
};

use crate::{font_reader::FontReader, Error, Font};

pub const fn create_font_renderer<F: Font>() -> FontRenderer {
    FontRenderer::new::<F>()
}

pub struct FontRenderer {
    font: FontReader,
}

impl FontRenderer {
    pub(crate) const fn new<FONT: Font>() -> Self {
        Self {
            font: FontReader::new::<FONT>(),
        }
    }

    pub fn render_glyph<Color, Display>(
        &self,
        ch: char,
        pos: Point,
        fg: Color,
        bg: Option<Color>,
        display: &mut Display,
    ) -> Result<i8, Error<Display::Error>>
    where
        Color: Clone + core::fmt::Debug,
        Display: DrawTarget<Color = Color>,
        Display::Error: core::fmt::Debug,
    {
        let bg = bg.unwrap();
        // if bg.is_some() && !self.font.supports_background_color {
        //     return Err(Error::BackgroundColorNotSupported);
        // }
        println!("{:#?}", self.font);

        let mut glyph = self.font.retrieve_glyph_data(ch)?;

        let topleft = glyph.topleft(&pos);
        let size = glyph.size();
        let advance = glyph.advance();

        let mut pixel_iter = {
            let mut num_zeros = glyph.read_runlength_0()?;
            let mut num_ones = glyph.read_runlength_1()?;
            let mut num_zeros_leftover = num_zeros;
            let mut num_ones_leftover = num_ones;
            move || -> Result<Color, Error<Display::Error>> {
                if num_zeros_leftover == 0 && num_ones_leftover == 0 {
                    let repeat = glyph.read_unsigned(1)? != 0;
                    if !repeat {
                        num_zeros = glyph.read_runlength_0()?;
                        num_ones = glyph.read_runlength_1()?;
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

                Ok(color)
            }
        };

        display
            .fill_contiguous(
                &Rectangle::new(topleft, size),
                std::iter::from_fn(move || Some(pixel_iter().unwrap())),
            )
            .map_err(Error::DisplayError)?;

        Ok(advance)
    }
}
