use embedded_graphics_core::{
    prelude::{PixelColor, Point},
    primitives::Rectangle,
};

/// The vertical rendering position of the font.
///
/// All of those texts are rendered at the green line, with different `VerticalPosition` settings:
///
/// ![Vertical text positions](data:image/png;base64,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)
///
/// Note that metrics like [`FontRenderer::get_ascent()`](crate::FontRenderer::get_ascent) or
/// [`FontRenderer::get_descent()`](crate::FontRenderer::get_descent)
/// are relative to [`VerticalPosition::Baseline`].
///
/// The default is [`VerticalPosition::Baseline`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalPosition {
    /// Anchored at the font baseline
    Baseline,
    /// Anchored at the top
    Top,
    /// Anchored at the center
    Center,
    /// Anchored at the bottom
    Bottom,
}

impl Default for VerticalPosition {
    fn default() -> Self {
        Self::Baseline
    }
}

/// The dimensions of a rendered glyph/text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedDimensions {
    /// The relative offset where a following glyph
    /// would have to get rendered.
    pub advance: Point,
    /// The bounding box of the rendered text/glyph.
    ///
    /// Can be `None` if nothing was rendered, like for
    /// a whitespace character.
    pub bounding_box: Option<Rectangle>,
}

impl RenderedDimensions {
    /// Creates an empty [`RenderedDimensions`] object with zero advance and no bounding box.
    pub const fn empty() -> Self {
        Self {
            advance: Point::new(0, 0),
            bounding_box: None,
        }
    }
}

/// The horizontal rendering position of the font.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    /// Anchored at the left side
    Left,
    /// Anchored at the center
    Center,
    /// Anchored at the right side
    Right,
}

/// The color of the rendered text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontColor<Color>
where
    Color: PixelColor,
{
    /// Only draw the text, do not touch the background.
    Transparent(Color),
    /// Draw the text and the background.
    ///
    /// Note that not all fonts support a background color.
    WithBackground {
        /// The foreground color
        fg: Color,
        /// The background color
        bg: Color,
    },
}

impl<Color> FontColor<Color>
where
    Color: PixelColor,
{
    pub(crate) fn has_background(&self) -> bool {
        matches!(self, Self::WithBackground { .. })
    }
}
