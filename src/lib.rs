//! This crate is a pure Rust reimplementation of the font subsystem of [U8g2](https://github.com/olikraus/u8g2).
//!
//! It is intended for the [embedded-graphics](https://crates.io/crates/embedded-graphics) ecosystem.
//!
//!
//! # Licensing
//!
//! While this crate is MIT / Apache-2.0 licensed, note that the fonts themselves *are not*.
//!
//! For more information about the font licenses, read the [license agreement](https://github.com/olikraus/u8g2/blob/master/LICENSE) of U8g2.
//!
//!
//! # Usage
//!
//! The central struct in this crate is the [`FontRenderer`]. It can render one specific font.
//!
//! A [`FontRenderer`] can be constructed through [`FontRenderer::new()`](FontRenderer::new). Its generic argument [`Font`] specifies which font it will render.
//!
//! Note that [`FontRenderer::new()`] is `const`, so it can be crated as a global variable at compile time for optimal performance.
//!
//! ## Fonts
//!
//! The fonts are directly imported from [U8g2](https://github.com/olikraus/u8g2/wiki).
//!
//! More information about all the available fonts can be found in the [U8g2 wiki](https://github.com/olikraus/u8g2/wiki/fntlistall).
//!
//!
//! ## Content Types
//!
//! Once constructed, the [`FontRenderer`] can render [the following objects](Content):
//!
//! - Characters: `'a'`
//! - Strings: `"Hello world!"`
//! - Format Strings: `format_args!("Nice: {}", 69)`
//!
//! ## Vertical Positioning
//!
//!
//!
//!
//!
//! ## Bounding box calculation
//!
//! Additional to the [`render()`](FontRenderer::render) and [`render_aligned()`](FontRenderer::render_aligned) methods,
//! there is also [`get_rendered_dimensions()`](FontRenderer::get_rendered_dimensions) and
//! [`get_rendered_dimensions_aligned()`](FontRenderer::get_rendered_dimensions_aligned).
//!
//! Those functions behave almost identical to their `render` counterparts, but don't actually perform any rendering. This
//! is very useful if the dimensions of the rendered text
//!
//!

#![no_std]
#![deny(missing_docs)]
#![doc(
    issue_tracker_base_url = "https://github.com/Finomnis/u8g2-fonts/issues",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code)))
)]

#[cfg(feature = "std")]
extern crate std;

mod content;
mod error;
mod font;
mod font_reader;
mod renderer;
mod utils;

/// A collection of [U8g2 fonts](https://github.com/olikraus/u8g2/wiki/fntlistall).
///
/// Note that every font has a different license. For more information, read the [U8g2 License Agreement](https://github.com/olikraus/u8g2/blob/master/LICENSE).
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub mod fonts;

/// Data types used in common API functions.
pub mod types;

pub use content::Content;
pub use error::Error;
pub use error::LookupError;
pub use font::Font;
pub use renderer::FontRenderer;
