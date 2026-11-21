/// Provides random access to font data.
///
/// Implementations must provide infallible reads for valid offsets.
/// Storage or I/O failures must not be represented by `None`;
/// implementations should panic if such failures occur.
///
/// The font data must remain unchanged while being used by a renderer.
pub trait FontData: Copy {
    /// Reads a byte at the given offset.
    ///
    /// Returns `None` if the offset is outside the available font data.
    ///
    /// Must not return `None` to indicate a storage or I/O failure.
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
