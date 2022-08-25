use super::Content;

pub struct TextContent<'a>(pub &'a str);

impl<'a> Content for TextContent<'a> {
    fn for_each_char<F, E>(&self, mut func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        for ch in self.0.chars() {
            func(ch)?;
        }

        Ok(())
    }

    fn for_each_char_infallible<F>(&self, func: F)
    where
        F: FnMut(char),
    {
        self.0.chars().for_each(func);
    }
}
