use embedded_graphics_core::prelude::Point;

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

    pub fn render_glyph<Color>(
        &self,
        ch: char,
        pos: Point,
        fg: Color,
        bg: Option<Color>,
    ) -> Result<i32, Error> {
        if bg.is_some() && !self.font.supports_background_color {
            return Err(Error::BackgroundColorNotSupported);
        }
        println!("{:#?}", self.font);

        println!("{:?}", self.font.retrieve_glyph_data(ch)?);

        todo!()
    }
}
