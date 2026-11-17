use crate::FontData;

/// Reads from a [`FontData`] and remembers the current position
pub(crate) struct Cursor<D> {
    data: D,
    pos: usize,
}

impl<D: FontData> Cursor<D> {
    /// Get the data relative to the current position
    /// without changing the current cursor position
    pub(crate) fn peek(&self, i: usize) -> u8 {
        self.data.get_byte(self.pos + i)
    }
    /// Move the cursor
    pub(crate) fn skip(&mut self, n: usize) {
        self.pos += n;
    }
    /// Retrieve the next byte and move the cursor by one
    pub(crate) fn next(&mut self) -> u8 {
        let val = self.data.get_byte(self.pos);
        self.pos += 1;
        val
    }
}
