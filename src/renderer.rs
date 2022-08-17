use embedded_graphics_core::prelude::{DrawTarget, Point};

use crate::{font_reader::FontReader, Error, Font};

/// Can render text of a specific [`Font`] to a [`DrawTarget`].
pub struct FontRenderer {
    font: FontReader,
}

impl FontRenderer {
    /// Creates a new instance of a font renderer.
    ///
    /// # Generics
    ///
    /// * `FONT` - the font to render. See [fonts](crate::fonts) for a list of available fonts
    ///            and refer to [U8g2](https://github.com/olikraus/u8g2/wiki/fntlistall) for a more detailed description of each font.
    pub const fn new<FONT: Font>() -> Self {
        Self {
            font: FontReader::new::<FONT>(),
        }
    }

    /// Renders a character glyph.
    ///
    /// Note that the background color is optional. Omitting it will render
    /// the character with a transparent background.
    ///
    /// Not every font supports a background color, some fonts require a transparent background.
    ///
    /// # Arguments
    ///
    /// * `ch` - The character to render.
    /// * `position` - The position to render to.
    /// * `foreground_color` - The foreground color.
    /// * `background_color` - The background color.
    /// * `display` - The display to render to.
    ///
    /// # Return
    ///
    /// The advance in pixels indicating the required offset to render the next character.
    ///
    pub fn render_glyph<Display>(
        &self,
        ch: char,
        position: Point,
        foreground_color: Display::Color,
        background_color: Option<Display::Color>,
        display: &mut Display,
    ) -> Result<i8, Error<Display::Error>>
    where
        Display: DrawTarget,
        Display::Error: core::fmt::Debug,
    {
        if background_color.is_some() && !self.font.supports_background_color {
            return Err(Error::BackgroundColorNotSupported);
        }

        let glyph = self.font.retrieve_glyph_data(ch)?;

        let advance = glyph.advance();
        let size = glyph.size();

        if size.width > 0 && size.height > 0 {
            let renderer = glyph.create_renderer();
            if let Some(background_color) = background_color {
                renderer.render_as_box_fill(
                    position,
                    display,
                    foreground_color,
                    background_color,
                )?;
            } else {
                renderer.render_transparent(position, display, foreground_color)?;
            }
        }

        Ok(advance)
    }
}
