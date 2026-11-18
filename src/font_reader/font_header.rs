use crate::FontData;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FontHeader {
    pub glyph_count: u8,
    pub supports_background_color: bool,
    pub m0: u8,
    pub m1: u8,
    pub bitcnt_w: u8,
    pub bitcnt_h: u8,
    pub bitcnt_x: u8,
    pub bitcnt_y: u8,
    pub bitcnt_d: u8,
    pub font_bounding_box_width: i8,
    pub font_bounding_box_height: i8,
    pub font_bounding_box_x_offset: i8,
    pub font_bounding_box_y_offset: i8,
    pub ascent: i8,
    pub descent: i8,
    pub ascent_of_parentheses: i8,
    pub descent_of_parentheses: i8,
    pub array_offset_upper_a: u16,
    pub array_offset_lower_a: u16,
    pub array_offset_0x0100: u16,
}

impl FontHeader {
    pub fn new<D: FontData>(data: D) -> Option<Self> {
        Some(Self {
            glyph_count: data.get_byte(0)?,
            supports_background_color: data.get_byte(1)? != 0,
            m0: data.get_byte(2)?,
            m1: data.get_byte(3)?,
            bitcnt_w: data.get_byte(4)?,
            bitcnt_h: data.get_byte(5)?,
            bitcnt_x: data.get_byte(6)?,
            bitcnt_y: data.get_byte(7)?,
            bitcnt_d: data.get_byte(8)?,
            font_bounding_box_width: data.get_byte(9)? as i8,
            font_bounding_box_height: data.get_byte(10)? as i8,
            font_bounding_box_x_offset: data.get_byte(11)? as i8,
            font_bounding_box_y_offset: data.get_byte(12)? as i8,
            ascent: data.get_byte(13)? as i8,
            descent: data.get_byte(14)? as i8,
            ascent_of_parentheses: data.get_byte(15)? as i8,
            descent_of_parentheses: data.get_byte(16)? as i8,
            array_offset_upper_a: u16::from_be_bytes([data.get_byte(17)?, data.get_byte(18)?]),
            array_offset_lower_a: u16::from_be_bytes([data.get_byte(19)?, data.get_byte(20)?]),
            array_offset_0x0100: u16::from_be_bytes([data.get_byte(21)?, data.get_byte(22)?]),
        })
    }
}
