#[cfg(feature = "avr-progmem")]
use avr_progmem::wrapper::ProgMem;

// keep flash pointers inside the library wrapper; they must never become rust slices.
#[derive(Clone, Copy, Debug)]
pub enum FontData {
    Slice(&'static [u8]),
    #[cfg(feature = "avr-progmem")]
    ProgMem {
        data: ProgMem<[u8]>,
        offset: usize,
    },
}

impl PartialEq for FontData {
    fn eq(&self, other: &Self) -> bool {
        let mut left = *self;
        let mut right = *other;
        core::iter::from_fn(|| left.next()).eq(core::iter::from_fn(|| right.next()))
    }
}

impl Eq for FontData {}

impl FontData {
    pub fn get(&self, index: usize) -> Option<u8> {
        match self {
            Self::Slice(data) => data.get(index).copied(),
            #[cfg(feature = "avr-progmem")]
            Self::ProgMem { data, offset } => {
                (index < data.len() - offset).then(|| data.load_at(offset + index))
            }
        }
    }

    pub fn skip(self, count: usize) -> Self {
        match self {
            Self::Slice(data) => Self::Slice(&data[count..]),
            #[cfg(feature = "avr-progmem")]
            Self::ProgMem { data, offset } => {
                assert!(count <= data.len() - offset);
                Self::ProgMem {
                    data,
                    offset: offset + count,
                }
            }
        }
    }

    pub fn next(&mut self) -> Option<u8> {
        let value = self.get(0)?;
        *self = self.skip(1);
        Some(value)
    }
}
