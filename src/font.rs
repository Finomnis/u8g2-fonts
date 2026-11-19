/// An abstract [U8g2](https://github.com/olikraus/u8g2/wiki/fntlistall) font interface.
///
/// Contains all information to create a [`FontRenderer`](crate::FontRenderer).
///
/// Implemented by [all available fonts](crate::fonts).
pub trait Font {
    /// The font data backend
    type FontDataType: FontData;
    /// The parsed font header
    const HEADER: FontHeader;
    /// The actual font data
    const DATA: Self::FontDataType;
}

macro_rules! font_definitions {
    (
        $(
            $(#[$attr:meta])*
            $fontname:ident
        ),* $(,)?
    ) => {
        $(
            $(#[$attr])*
            #[doc = concat!(r#"<img src="https://raw.githubusercontent.com/wiki/olikraus/u8g2/fntpic/"#, stringify!($fontname), r#".png">"#)]
            pub struct $fontname;

            $(#[$attr])*
            impl $crate::Font for $fontname {
                type FontDataType = &'static [u8];
                const HEADER: $crate::FontHeader = $crate::FontHeader::from_raw(Self::DATA);
                const DATA: &'static [u8] = include_bytes!(concat!(stringify!($fontname), ".u8g2font"));
            }
        )*
    };
}

pub(crate) use font_definitions;

use crate::{FontData, FontHeader};
