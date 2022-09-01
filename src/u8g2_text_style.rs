use embedded_graphics::text::{
    renderer::{CharacterStyle, TextMetrics, TextRenderer},
    Baseline,
};

use embedded_graphics_core::prelude::{DrawTarget, PixelColor, Point};

use crate::{
    types::{FontColor, VerticalPosition},
    Font, FontRenderer,
};

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

/// TODO
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
            font: FontRenderer::new::<F>(),
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
        let result = if let Some(text_color) = self.text_color {
            let color = if let Some(background_color) = self.background_color {
                FontColor::WithBackground {
                    fg: text_color,
                    bg: background_color,
                }
            } else {
                FontColor::Transparent(text_color)
            };
            self.font
                .render(text, position, baseline.into(), color, target)
        } else {
            self.font
                .get_rendered_dimensions(text, position, baseline.into())
                .map_err(Into::into)
        };
        todo!()
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
        todo!()
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        todo!()
    }

    fn line_height(&self) -> u32 {
        self.font.get_line_height()
    }
}

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
