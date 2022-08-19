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
        None
    }

    fn next_entry(&mut self) -> Option<UnicodeJumptableEntry> {
        None
    }
}
