use crate::Font;

pub struct FontRenderer {
    data: &'static [u8],
}

impl FontRenderer {
    pub(crate) const fn new<FONT: Font>() -> Self {
        Self { data: &FONT::DATA }
    }

    pub fn a(&self) {
        println!("A: {:?}", self.data);
    }
}
