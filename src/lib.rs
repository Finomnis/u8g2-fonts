#![no_std]

mod error;
mod font;
mod font_reader;
mod glyph_reader;
mod glyph_renderer;
mod glyph_searcher;
mod renderer;

/// A collection of [U8g2 fonts](https://github.com/olikraus/u8g2/wiki/fntlistall).
///
/// Note that every font has a different license. For more information, read the [U8g2 License Agreement](https://github.com/olikraus/u8g2/blob/master/LICENSE).
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
