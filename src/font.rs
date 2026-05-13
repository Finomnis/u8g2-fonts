/// An abstract [U8g2](https://github.com/olikraus/u8g2/wiki/fntlistall) font interface.
///
/// Contains all information to create a [`FontRenderer`](crate::FontRenderer).
///
/// Implemented by [all available fonts](crate::fonts).
pub struct Font(#[doc(hidden)] pub &'static [u8]);

macro_rules! font_definitions {
    ( $($fontname:ident),* $(,)? ) => {
        $(
            #[doc = concat!(r#"<img src="https://raw.githubusercontent.com/wiki/olikraus/u8g2/fntpic/"#, stringify!($fontname), r#".png">"#)]
            pub const $fontname: $crate::Font = $crate::Font(include_bytes!(concat!(stringify!($fontname), ".u8g2font")));
        )*
    };
}

pub(crate) use font_definitions;
