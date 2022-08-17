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

        let mut glyph = self.font.retrieve_glyph_data(ch)?;

        let glyph_width = glyph.read_unsigned(self.font.bitcnt_w)?;
        let glyph_height = glyph.read_unsigned(self.font.bitcnt_h)?;

        let x = glyph.read_signed(self.font.bitcnt_x)?;
        let y = glyph.read_signed(self.font.bitcnt_y)?;
        let d = glyph.read_signed(self.font.bitcnt_d)?;

        dbg!(glyph_width);
        dbg!(glyph_height);
        dbg!(x);
        dbg!(y);
        dbg!(d);

        todo!()
    }
}
