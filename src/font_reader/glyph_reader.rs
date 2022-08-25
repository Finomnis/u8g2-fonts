use embedded_graphics_core::prelude::{Point, Size};

use crate::{
    font_reader::{glyph_renderer::GlyphRenderer, FontReader},
    utils::DebugIgnore,
    LookupError,
};

#[derive(Clone, Debug)]
pub struct GlyphReader {
    data: DebugIgnore<&'static [u8]>,
    bit_pos: u8,
    current_byte: u8,
    glyph_width: u8,
    glyph_height: u8,
    offset_x: i8,
    offset_y: i8,
    advance: i8,
    bitcount_0: u8,
    bitcount_1: u8,
}

impl GlyphReader {
    pub fn new(data: &'static [u8], font: &FontReader) -> Result<Self, LookupError> {
        let mut this = Self {
            data: DebugIgnore(data),
            // Start at 8 to mark current_byte as invalid
            bit_pos: 8,
            current_byte: 0,
            glyph_width: 0,
            glyph_height: 0,
            offset_x: 0,
            offset_y: 0,
            advance: 0,
            bitcount_0: font.m0,
            bitcount_1: font.m1,
        };

        this.glyph_width = this.read_unsigned(font.bitcnt_w)?;
        this.glyph_height = this.read_unsigned(font.bitcnt_h)?;

        this.offset_x = this.read_signed(font.bitcnt_x)?;
        this.offset_y = this.read_signed(font.bitcnt_y)?;
        this.advance = this.read_signed(font.bitcnt_d)?;

        Ok(this)
    }

    pub fn read_unsigned(&mut self, bits: u8) -> Result<u8, LookupError> {
        let bit_start = self.bit_pos;
        let mut bit_end = bit_start + bits;

        // Read from current byte
        let mut value = self.current_byte.overflowing_shr(bit_start as u32).0;

        // If necessary, fetch next byte
        if bit_end >= 8 {
            let value2 = *self.data.first().ok_or(LookupError::InternalError)?;
            *self.data = self.data.get(1..).ok_or(LookupError::InternalError)?;
            bit_end -= 8;
            self.current_byte = value2;

            value |= value2.overflowing_shl((8 - bit_start) as u32).0;
        }

        self.bit_pos = bit_end;

        let out = value & (((1u16 << bits) - 1) as u8);
        Ok(out)
    }

    pub fn read_signed(&mut self, bits: u8) -> Result<i8, LookupError> {
        self.read_unsigned(bits)
            .map(|v| (v as i8).wrapping_sub(1 << (bits - 1)))
    }

    pub fn topleft(&self, pos: &Point) -> Point {
        Point::new(
            pos.x + self.offset_x as i32,
            pos.y - (self.glyph_height as i32 + self.offset_y as i32),
        )
    }

    pub fn size(&self) -> Size {
        Size::new(self.glyph_width as u32, self.glyph_height as u32)
    }

    pub fn advance(&self) -> i8 {
        self.advance
    }

    pub fn read_runlength_0(&mut self) -> Result<u8, LookupError> {
        self.read_unsigned(self.bitcount_0)
    }

    pub fn read_runlength_1(&mut self) -> Result<u8, LookupError> {
        self.read_unsigned(self.bitcount_1)
    }

    pub fn create_renderer(&self) -> GlyphRenderer {
        GlyphRenderer::new(self)
    }
}
