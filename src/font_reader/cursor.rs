use crate::{utils::DebugIgnore, FontData};

/// Reads from a [`FontData`] and remembers the current position
#[derive(Debug, Clone, Copy)]
pub(crate) struct Cursor<D: Copy> {
    data: DebugIgnore<D>,
    pos: usize,
}

impl<D: Copy> Cursor<D> {
    pub fn new(data: D, pos: usize) -> Self {
        Self {
            data: DebugIgnore(data),
            pos,
        }
    }
}

impl<D: FontData> Cursor<D> {
    /// Get the data relative to the current position
    /// without changing the current cursor position
    pub(crate) fn peek(&self, i: usize) -> Option<u8> {
        self.data.get_byte(self.pos + i)
    }
    /// Move the cursor
    pub(crate) fn skip(&mut self, n: usize) {
        self.pos += n;
    }
    /// Retrieve the next byte and move the cursor by one
    pub(crate) fn next(&mut self) -> Option<u8> {
        let val = self.data.get_byte(self.pos);
        self.pos += 1;
        val
    }
}
