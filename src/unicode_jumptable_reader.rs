pub struct UnicodeJumptableEntry {
    jump_distance: u16,
    character_upper_limit: u16,
}

pub struct UnicodeJumptableReader {
    data: &'static [u8],
}

impl UnicodeJumptableReader {
    pub fn new(data: &'static [u8]) -> Self {
        Self { data }
    }

    pub fn calculate_jump_offset(mut self, encoding: u16) -> Option<usize> {
        let mut jump_offset = 0;

        while {
            let entry = self.next_entry()?;
            jump_offset += entry.jump_distance as usize;
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
        let value = *self.data.get(0)?;
        self.data = self.data.get(1..)?;
        Some(value)
    }
}
