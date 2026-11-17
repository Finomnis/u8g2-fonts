/// Can retrieve the data of a font
pub trait FontData: Copy {
    /// Fetch the font at the given position
    fn get_byte(&self, offset: usize) -> Option<u8>;
}

impl<'a> FontData for &'a [u8] {
    fn get_byte(&self, offset: usize) -> Option<u8> {
        self.get(offset).copied()
    }
}

impl<T> FontData for &T
where
    T: FontData + ?Sized,
{
    fn get_byte(&self, offset: usize) -> Option<u8> {
        T::get_byte(self, offset)
    }
}
