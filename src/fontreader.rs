use crate::Font;

#[derive(Debug)]
pub struct FontReader {
    data: &'static [u8],
    supports_background_color: bool,
    glyph_count: u8,
    m0: u8,
    m1: u8,
    bitcntW: u8,
    bitcntH: u8,
    bitcntX: u8,
    bitcntY: u8,
    bitcntD: u8,
    font_bounding_box_width: i8,
    font_bounding_box_height: i8,
    font_bounding_box_x_offset: i8,
    font_bounding_box_y_offset: i8,
    ascent: i8,
    descent: i8,
    ascent_of_parantheses: i8,
    descent_of_parantheses: i8,
    array_offset_A: u16,
    array_offset_a: u16,
    array_offset_0x0100: u16,
}

impl FontReader {
    pub const fn new<F: Font>() -> Self {
        let data = F::DATA;

        Self {
            data,
            glyph_count: data[0],
            supports_background_color: data[1] != 0,
            m0: data[2],
            m1: data[3],
            bitcntW: data[4],
            bitcntH: data[5],
            bitcntX: data[6],
            bitcntY: data[7],
            bitcntD: data[8],
            font_bounding_box_width: data[9] as i8,
            font_bounding_box_height: data[10] as i8,
            font_bounding_box_x_offset: data[11] as i8,
            font_bounding_box_y_offset: data[12] as i8,
            ascent: data[13] as i8,
            descent: data[14] as i8,
            ascent_of_parantheses: data[15] as i8,
            descent_of_parantheses: data[16] as i8,
            array_offset_A: u16::from_be_bytes([data[17], data[18]]),
            array_offset_a: u16::from_be_bytes([data[19], data[20]]),
            array_offset_0x0100: u16::from_be_bytes([data[21], data[22]]),
        }
    }
}
