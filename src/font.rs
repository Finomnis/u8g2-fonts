/// An abstract [U8g2](https://github.com/olikraus/u8g2/wiki/fntlistall) font interface.
///
/// Contains all information to create a [`FontRenderer`](crate::FontRenderer).
///
/// Implemented by [all available fonts](crate::fonts).
pub trait Font {
    #[doc(hidden)]
    const DATA: &'static [u8];
}

/// A marker trait that indicates that a font supports background colors.
///
/// Many fonts do not support this, because their glyphs overlap.
pub trait SupportsBackgroundColor {}
