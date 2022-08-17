#![no_std]

mod error;
mod font;
mod font_reader;
mod glyph_reader;
mod glyph_renderer;
mod glyph_searcher;
mod renderer;

/// A collection of fonts.
///
/// For more information about the specific fonts, visit the [U8g2 wiki](https://github.com/olikraus/u8g2/wiki/fntlistall).
pub mod fonts;

pub use error::Error;
pub use font::Font;
pub use renderer::FontRenderer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
