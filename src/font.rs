/// An abstract [U8g2](https://github.com/olikraus/u8g2/wiki/fntlistall) font interface.
///
/// Contains all information to create a [FontRenderer](crate::FontRenderer).
pub trait Font {
    #[doc(hidden)]
    const DATA: &'static [u8];
}
