use embedded_graphics_core::prelude::Point;

use crate::{
    font_reader::FontReader, glyph_reader::GlyphReader, glyph_searcher::GlyphSearcher, Error, Font,
};

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

    fn retrieve_glyph_data(&self, ch: char) -> Result<GlyphReader, Error> {
        // Retrieve u16 glyph value
        let encoding = u16::try_from(ch as u32).map_err(|_| Error::GLYPH_NOT_FOUND(ch))?;

        let mut glyph = GlyphSearcher::new(&self.font);

        println!("Searching for glyph {}", ch);

        if encoding <= 255 {
            if encoding >= b'a' as u16 {
                if !glyph.jump_by(self.font.array_offset_lower_a) {
                    return Err(Error::GLYPH_NOT_FOUND(ch));
                };
            } else if encoding >= b'A' as u16 {
                if !glyph.jump_by(self.font.array_offset_upper_a) {
                    return Err(Error::GLYPH_NOT_FOUND(ch));
                };
            }

            while glyph.get_ch()? as u16 != encoding {
                if !glyph.jump_to_next()? {
                    return Err(Error::GLYPH_NOT_FOUND(ch));
                }
            }

            glyph.into_glyph_reader()
        } else {
            // TODO: Support Unicode
            Err(Error::GLYPH_NOT_FOUND(ch))
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
            return Err(Error::BACKGROUND_COLOR_NOT_SUPPORTED);
        }
        println!("{:#?}", self.font);

        println!("{:?}", self.retrieve_glyph_data(ch)?);

        todo!()
    }
}
