/// The vertical rendering position of the font.
///
/// Note that metrics like [`FontRenderer::get_glyph_bounding_box()`](crate::FontRenderer::get_glyph_bounding_box),
/// [`FontRenderer::get_ascent()`](crate::FontRenderer::get_ascent) or
/// [`FontRenderer::get_descent()`](crate::FontRenderer::get_descent)
/// are relative to [`FontPos::Baseline`].
///
/// The default is [`FontPos::Baseline`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontPos {
    /// Anchored at the font baseline
    Baseline,
    /// Anchored at the top
    Top,
    /// Anchored at the center
    Center,
    /// Anchored at the bottom
    Bottom,
}

impl Default for FontPos {
    fn default() -> Self {
        Self::Baseline
    }
}
