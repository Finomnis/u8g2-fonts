use embedded_graphics_core::prelude::{DrawTarget, PixelColor, Point};

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
        Color: core::fmt::Debug + PixelColor,
        Display: DrawTarget<Color = Color>,
        Display::Error: core::fmt::Debug,
    {
        // if bg.is_some() && !self.font.supports_background_color {
        //     return Err(Error::BackgroundColorNotSupported);
        // }

        let glyph = self.font.retrieve_glyph_data(ch)?;
        let renderer = glyph.create_renderer();
        if let Some(bg) = bg {
            renderer.render_as_box_fill(pos, display, fg, bg)?;
        } else {
            renderer.render_transparent(pos, display, fg)?;
        }

        Ok(glyph.advance())
    }
}
