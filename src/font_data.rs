/// Can retrieve the data of a font
pub trait FontData: Copy {
    /// Read a byte from the font data at the given offset
    fn get_byte(&self, offset: usize) -> Option<u8>;
}

impl FontData for &[u8] {
    #[inline]
    fn get_byte(&self, offset: usize) -> Option<u8> {
        self.get(offset).copied()
    }
}

impl<T> FontData for &T
where
    T: FontData,
{
    #[inline]
    fn get_byte(&self, offset: usize) -> Option<u8> {
        T::get_byte(self, offset)
    }
}
