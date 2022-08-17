use crate::Error;

#[derive(Debug)]
pub struct GlyphReader {
    data: &'static [u8],
    bit_pos: u8,
    current_byte: u8,
}

impl GlyphReader {
    pub fn new(data: &'static [u8]) -> Self {
        Self {
            data,
            // Start at 8 to mark current_byte as invalid
            bit_pos: 8,
            current_byte: 0,
        }
    }

    pub fn read_unsigned<DisplayError>(&mut self, bits: u8) -> Result<u8, Error<DisplayError>> {
        let bit_start = self.bit_pos;
        let mut bit_end = bit_start + bits;

        // Read from current byte
        let mut value = self.current_byte.overflowing_shr(bit_start as u32).0;

        // If necessary, fetch next byte
        if bit_end >= 8 {
            let value2 = *self.data.get(0).ok_or(Error::InternalError)?;
            self.data = self.data.get(1..).ok_or(Error::InternalError)?;
            bit_end -= 8;
            self.current_byte = value2;

            value |= value2.overflowing_shl((8 - bit_start) as u32).0;
        }

        self.bit_pos = bit_end;

        let out = value & (((1u16 << bits) - 1) as u8);
        Ok(out)
    }

    pub fn read_signed<DisplayError>(&mut self, bits: u8) -> Result<i8, Error<DisplayError>> {
        self.read_unsigned(bits)
            .map(|v| (v as i8).wrapping_sub(1 << (bits - 1)))
    }
}
