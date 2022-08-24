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
//! # Rendering
//!
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

pub use error::Error;
pub use error::LookupError;
pub use font::Font;
pub use renderer::FontRenderer;
