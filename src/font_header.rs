use crate::FontData;

/// The parsed header of the font
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FontHeader {
    pub(crate) glyph_count: u8,
    pub(crate) supports_background_color: bool,
    pub(crate) m0: u8,
    pub(crate) m1: u8,
    pub(crate) bitcnt_w: u8,
    pub(crate) bitcnt_h: u8,
    pub(crate) bitcnt_x: u8,
    pub(crate) bitcnt_y: u8,
    pub(crate) bitcnt_d: u8,
    pub(crate) font_bounding_box_width: i8,
    pub(crate) font_bounding_box_height: i8,
    pub(crate) font_bounding_box_x_offset: i8,
    pub(crate) font_bounding_box_y_offset: i8,
    pub(crate) ascent: i8,
    pub(crate) descent: i8,
    pub(crate) ascent_of_parentheses: i8,
    pub(crate) descent_of_parentheses: i8,
    pub(crate) array_offset_upper_a: u16,
    pub(crate) array_offset_lower_a: u16,
    pub(crate) array_offset_0x0100: u16,
}

impl FontHeader {
    /// The size of the font header
    pub const RAW_SIZE: usize = 23;

    /// Creates a [`FontHeader`] from raw header data
    pub const fn from_raw(data: &[u8]) -> Self {
        let this = Self {
            glyph_count: data[0],
            supports_background_color: data[1] != 0,
            m0: data[2],
            m1: data[3],
            bitcnt_w: data[4],
            bitcnt_h: data[5],
            bitcnt_x: data[6],
            bitcnt_y: data[7],
            bitcnt_d: data[8],
            font_bounding_box_width: data[9] as i8,
            font_bounding_box_height: data[10] as i8,
            font_bounding_box_x_offset: data[11] as i8,
            font_bounding_box_y_offset: data[12] as i8,
            ascent: data[13] as i8,
            descent: data[14] as i8,
            ascent_of_parentheses: data[15] as i8,
            descent_of_parentheses: data[16] as i8,
            array_offset_upper_a: u16::from_be_bytes([data[17], data[18]]),
            array_offset_lower_a: u16::from_be_bytes([data[19], data[20]]),
            array_offset_0x0100: u16::from_be_bytes([data[21], data[22]]),
        };

        assert!(this.font_bounding_box_height >= 0);
        assert!(this.font_bounding_box_width >= 0);

        this
    }
    /// Reads a [`FontHeader`] from the start of [`FontData`].
    pub fn from_fontdata<D: FontData>(data: D) -> Option<Self> {
        let mut header_raw = [0u8; FontHeader::RAW_SIZE];
        for (pos, val) in header_raw.iter_mut().enumerate() {
            *val = data.get_byte(pos)?;
        }
        Some(Self::from_raw(&header_raw))
    }

    /// Reads the default line height for the font
    pub const fn get_default_line_height(&self) -> u8 {
        assert!(self.font_bounding_box_height >= 0);
        self.font_bounding_box_height as u8 + 1
    }
}
