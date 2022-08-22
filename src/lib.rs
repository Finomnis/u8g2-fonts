//!
//!

#![no_std]
#![deny(missing_docs)]
#![doc(
    issue_tracker_base_url = "https://github.com/Finomnis/u8g2-fonts/issues",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code)))
)]

mod error;
mod font;
mod font_reader;
mod glyph_reader;
mod glyph_renderer;
mod glyph_searcher;
mod renderer;
mod unicode_jumptable_reader;
mod utils;

/// A collection of [U8g2 fonts](https://github.com/olikraus/u8g2/wiki/fntlistall).
///
/// Note that every font has a different license. For more information, read the [U8g2 License Agreement](https://github.com/olikraus/u8g2/blob/master/LICENSE).
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub mod fonts;

/// Data types used in common API functions.
pub mod types;

pub use error::DrawError;
pub use error::Error;
pub use font::Font;
pub use renderer::FontRenderer;
