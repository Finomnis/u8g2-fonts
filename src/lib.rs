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
//! ## Positioning and Alignment
//!
//! The [`FontRenderer::render()`](FontRenderer::render) allows for basic vertical positioning. Horizontally, it renders exactly like specified in the font.
//!
//! For more advanced usecases, use the [`FontRenderer::render_aligned()`](FontRenderer::render_aligned) method.
//! It further allows for horizontal alignment through an aditional parameter.
//!
//! ## Bounding Box Calculation
//!
//! Additional to the [`render()`](FontRenderer::render) and [`render_aligned()`](FontRenderer::render_aligned) methods,
//! there is also [`get_rendered_dimensions()`](FontRenderer::get_rendered_dimensions) and
//! [`get_rendered_dimensions_aligned()`](FontRenderer::get_rendered_dimensions_aligned).
//!
//! Those functions behave almost identical to their `render` counterparts, but don't actually perform any rendering. This
//! can be very useful if the dimensions of the text are required for other drawing operations prior to the actual text rendering.
//!
//! ## Colors and Backgrounds
//!
//! While a foreground color must always be specified for rendering a font, there is also the option to set a background color.
//! This is mainly for monospace fonts.
//!
//! Note that many fonts do not actually support rendering with a background color (due to occlusions).
//! Supplying a background color to a font that doesn't support it causes a runtime error.
//!
//! # Example
//!
//! ```rust
//! # use u8g2_fonts::types::*;
//! # use u8g2_fonts::FontRenderer;
//! # use u8g2_fonts::fonts;
//! # use embedded_graphics_core::prelude::*;
//! # use embedded_graphics_core::pixelcolor::BinaryColor;
//! # pub fn render<Display>(mut display: Display)
//! # where
//! #    Display: DrawTarget<Color = BinaryColor>,
//! #    Display::Error: core::fmt::Debug
//! # {
//! let font = FontRenderer::new::<fonts::u8g2_font_haxrcorp4089_t_cyrillic>();
//!
//! font.render_aligned(
//!     format_args!("Answer: {}", 42),
//!     display.bounding_box().center() + Point::new(0, 16),
//!     VerticalPosition::Baseline,
//!     HorizontalAlignment::Center,
//!     FontColor::Transparent(BinaryColor::On),
//!     &mut display,
//! )
//! .unwrap();
//! # }
//! ```

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

#[cfg(feature = "embedded_graphics_font_integration")]
mod u8g2_text_style;

#[cfg(feature = "embedded_graphics_font_integration")]
pub use u8g2_text_style::U8g2TextStyle;
