#[derive(Debug)]
pub struct GlyphReader {
    data: &'static [u8],
}

impl GlyphReader {
    pub fn new(data: &'static [u8]) -> Self {
        Self { data }
    }
}
