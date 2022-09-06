use embedded_graphics::text::{
    renderer::{CharacterStyle, TextMetrics, TextRenderer},
    Baseline,
};

use embedded_graphics_core::{
    prelude::{DrawTarget, PixelColor, Point, Size},
    primitives::Rectangle,
};

use crate::{
    types::{FontColor, VerticalPosition},
    Error, Font, FontRenderer,
};

#[cfg_attr(docsrs, doc(cfg(feature = "embedded_graphics_textstyle")))]
impl From<Baseline> for VerticalPosition {
    fn from(baseline: Baseline) -> Self {
        match baseline {
            Baseline::Top => VerticalPosition::Top,
            Baseline::Bottom => VerticalPosition::Bottom,
            Baseline::Middle => VerticalPosition::Center,
            Baseline::Alphabetic => VerticalPosition::Baseline,
        }
    }
}

/// Provides a character style object for drawing text with [`embedded_graphics::text::Text`].
///
#[cfg_attr(docsrs, doc(cfg(feature = "embedded_graphics_textstyle")))]
#[derive(Debug, Clone)]
pub struct U8g2TextStyle<C> {
    /// Text color.
    pub text_color: Option<C>,
    /// Background color.
    pub background_color: Option<C>,
    /// The font renderer
    font: FontRenderer,
}

impl<C> U8g2TextStyle<C> {
    /// Creates a text style with transparent background.
    pub fn new<F: Font>(font: F, text_color: C) -> Self {
        drop(font);
        Self {
            text_color: Some(text_color),
            background_color: None,
            font: FontRenderer::new::<F>().with_ignore_unknown_chars(true),
        }
    }
}

impl<C> TextRenderer for U8g2TextStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn draw_string<D>(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        // For some reason, font baseline in embedded-graphics seems to be shifted by one
        let mut adjusted_position = position;
        if let Baseline::Alphabetic = baseline {
            adjusted_position.y += 1;
        }

        let result;

        if let Some(text_color) = self.text_color {
            let color = match self.background_color {
                Some(background_color) => FontColor::WithBackground {
                    fg: text_color,
                    bg: background_color,
                },
                None => FontColor::Transparent(text_color),
            };
            result = self
                .font
                .render(text, adjusted_position, baseline.into(), color, target);
        } else {
            result = self
                .font
                .get_rendered_dimensions(text, adjusted_position, baseline.into())
                .map_err(Into::into);
        };

        result
            .map_err(unwrap_error)
            .map(|dims| position + dims.advance)
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        if width != 0 {
            if let Some(color) = self.background_color {
                let whitespace_dimensions = self.font
                    .get_rendered_dimensions(' ', position, baseline.into())
                    .expect("Internal error: Should never happen because the font reader is configured to ignore unknown characters.");

                if let Some(bounding_box) = whitespace_dimensions.bounding_box {
                    let top_left = bounding_box.top_left;
                    let height = bounding_box.size.height;

                    target
                        .fill_solid(&Rectangle::new(top_left, Size::new(width, height)), color)?;
                }
            }
        }

        Ok(position + Point::new(width as i32, 0))
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        // For some reason, font baseline in embedded-graphics seems to be shifted by one
        let mut adjusted_position = position;
        if let Baseline::Alphabetic = baseline {
            adjusted_position.y += 1;
        }

        let dims = self
            .font
            .get_rendered_dimensions(text, adjusted_position, baseline.into())
            .unwrap();
        TextMetrics {
            bounding_box: dims.bounding_box.unwrap_or_default(),
            next_position: position + dims.advance,
        }
    }

    fn line_height(&self) -> u32 {
        self.font.get_default_line_height()
    }
}

/// Note that `U8g2TextStyle` does not support underline or strikethrough,
/// so [`set_underline_color()`](CharacterStyle::set_underline_color) and
/// [`set_strikethrough_color`](CharacterStyle::set_strikethrough_color()) does nothing.
impl<C> CharacterStyle for U8g2TextStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn set_text_color(&mut self, text_color: Option<Self::Color>) {
        self.text_color = text_color;
    }

    fn set_background_color(&mut self, background_color: Option<Self::Color>) {
        self.background_color = background_color;
    }
}

impl<C> TextRenderer for &U8g2TextStyle<C>
where
    C: PixelColor,
{
    type Color = C;

    fn draw_string<D>(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        (*self).draw_string(text, position, baseline, target)
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        (*self).draw_whitespace(width, position, baseline, target)
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        (*self).measure_string(text, position, baseline)
    }

    fn line_height(&self) -> u32 {
        (*self).line_height()
    }
}

// Extracted to separate function for easier testability
fn unwrap_error<E>(err: Error<E>) -> E {
    match err {
        Error::DisplayError(e) => e,
        Error::BackgroundColorNotSupported => {
            panic!("Background color not supported for this font!")
        }
        Error::GlyphNotFound(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::println;

    use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};

    use crate::fonts;

    use super::*;

    fn examine<T: Clone + core::fmt::Debug>(val: T) {
        println!("{:?}", val.clone());
    }

    #[test]
    fn is_debug_and_clone() {
        examine(U8g2TextStyle::new(fonts::u8g2_font_10x20_mf, Rgb888::RED));
    }

    // Dummy test for coverage; coverage requires unreachable!() branches to be covered,
    // which doesn't make much sense, so we trick it here.
    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn dummy_test_unreachable() {
        unwrap_error(Error::<&str>::GlyphNotFound('a'));
    }
}
