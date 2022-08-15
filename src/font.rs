use crate::FontRenderer;

pub trait Font {
    const DATA: &'static [u8];
}
