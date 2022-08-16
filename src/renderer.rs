use embedded_graphics_core::prelude::Point;

use crate::{font_reader::FontReader, glyph_reader::GlyphReader, Error, Font};

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

    fn retrieve_glyph_data(
        &self,
        ch: char,
    ) -> Result<GlyphReader<crate::glyph_reader::READ_MODE>, Error> {
        // Retrieve u16 glyph value
        let encoding = {
            let mut utf16_data = [0u16; 2];
            ch.encode_utf16(&mut utf16_data);
            if utf16_data[1] != 0 {
                return Err(Error::GLYPH_NOT_FOUND(ch));
            }
            utf16_data[0]
        };

        let mut glyph = GlyphReader::new(&self.font);

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

            println!("Glyph found!");
            // if ( encoding <= 255 )
            // {
            //     if ( encoding >= 'a' )
            //     {
            //     font += u8g2->font_info.start_pos_lower_a;
            //     }
            //     else if ( encoding >= 'A' )
            //     {
            //     font += u8g2->font_info.start_pos_upper_A;
            //     }

            //     for(;;)
            //     {
            //     if ( u8x8_pgm_read( font + 1 ) == 0 )
            //     break;
            //     if ( u8x8_pgm_read( font ) == encoding )
            //     {
            //     return font+2;	/* skip encoding and glyph size */
            //     }
            //     font += u8x8_pgm_read( font + 1 );
            //     }
            // }

            todo!()
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

        self.retrieve_glyph_data(ch)?;

        todo!()
    }
}
