use super::Content;

pub struct GlyphContent(pub char);

impl Content for GlyphContent {
    fn for_each_char<F, E>(&self, mut func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        func(self.0)
    }

    fn for_each_char_infallible<F>(&self, mut func: F)
    where
        F: FnMut(char),
    {
        func(self.0)
    }

    fn get_newline_count(&self) -> u32 {
        0
    }
}
