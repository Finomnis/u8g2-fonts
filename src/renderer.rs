use crate::Font;

pub const fn create_font_renderer<F: Font>() -> FontRenderer {
    FontRenderer::new::<F>()
}

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
