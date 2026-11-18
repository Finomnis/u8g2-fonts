use super::data::FontData;

pub struct UnicodeJumptableEntry {
    jump_distance: u16,
    character_upper_limit: u16,
}

pub struct UnicodeJumptableReader {
    data: FontData,
}

impl UnicodeJumptableReader {
    pub fn new(data: FontData) -> Self {
        Self { data }
    }

    pub fn calculate_jump_offset(mut self, encoding: u16) -> Option<usize> {
        let mut jump_offset = 0;

        while {
            let entry = self.next_entry()?;
            jump_offset += usize::from(entry.jump_distance);
            entry.character_upper_limit < encoding
        } {}

        Some(jump_offset)
    }

    fn next_entry(&mut self) -> Option<UnicodeJumptableEntry> {
        let jump_distance = u16::from_be_bytes([self.next_byte()?, self.next_byte()?]);
        let character_upper_limit = u16::from_be_bytes([self.next_byte()?, self.next_byte()?]);
        Some(UnicodeJumptableEntry {
            jump_distance,
            character_upper_limit,
        })
    }

    fn next_byte(&mut self) -> Option<u8> {
        self.data.next()
    }
}
