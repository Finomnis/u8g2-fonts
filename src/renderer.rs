use embedded_graphics_core::{
    prelude::{DrawTarget, Point, Size},
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
        Color: Clone,
        Display: DrawTarget<Color = Color>,
    {
        // if bg.is_some() && !self.font.supports_background_color {
        //     return Err(Error::BackgroundColorNotSupported);
        // }
        println!("{:#?}", self.font);

        let glyph = self.font.retrieve_glyph_data(ch)?;

        let topleft = glyph.topleft(&pos);
        let size = glyph.size();
        let advance = glyph.advance();

        display
            .fill_contiguous(
                &Rectangle::new(topleft, size),
                std::iter::from_fn(move || bg.clone()),
            )
            .map_err(Error::DisplayError)?;

        Ok(advance)
    }
}
