/// An abstract [U8g2](https://github.com/olikraus/u8g2/wiki/fntlistall) font interface.
///
/// Contains all information to create a [`FontRenderer`](crate::FontRenderer).
///
/// Implemented by [all available fonts](crate::fonts).
pub trait Font {
    #[doc(hidden)]
    const DATA: &'static [u8];
}

macro_rules! font_definitions {
    ( $($fontname:ident),* $(,)? ) => {
        $(
            pub struct $fontname;
            impl $crate::Font for $fontname {
                const DATA: &'static [u8] = include_bytes!(concat!(stringify!($fontname), ".u8g2font"));
            }
        )*
    };
}

pub(crate) use font_definitions;
