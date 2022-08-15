use lazy_static::lazy_static;
use miette::{IntoDiagnostic, Result, WrapErr};
use regex::bytes::{CaptureMatches, Regex};

pub struct FontEntry<'a> {
    pub name: &'a [u8],
    pub c_data: &'a [u8],
    pub expected_length: usize,
}

pub struct FontEntryIter<'a> {
    match_iter: CaptureMatches<'static, 'a>,
}

lazy_static! {
    static ref FONT_REGEX: Regex = Regex::new(
        r#"const uint8_t (\w*)\[(\d*)\] U8G2_FONT_SECTION\("(\w*)"\) =((?:\s*"[^"]*")*);"#,
    )
    .unwrap();
}

impl<'a> FontEntryIter<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self> {
        Ok(Self {
            match_iter: FONT_REGEX.captures_iter(data),
        })
    }
}

impl<'a> Iterator for FontEntryIter<'a> {
    type Item = Result<FontEntry<'a>>;

    fn next(&mut self) -> Option<Result<FontEntry<'a>>> {
        self.match_iter.next().map(|font_match| {
            let name = font_match.get(1).unwrap().as_bytes();
            let expected_length: usize =
                String::from_utf8(font_match.get(2).unwrap().as_bytes().to_vec())
                    .into_diagnostic()
                    .wrap_err("Unable to read font length")?
                    .parse()
                    .into_diagnostic()
                    .wrap_err("Unable to read font length")?;
            let name2 = font_match.get(3).unwrap().as_bytes();
            let c_data = font_match.get(4).unwrap().as_bytes();

            assert!(name == name2);

            Ok(FontEntry {
                name,
                c_data,
                expected_length,
            })
        })
    }
}
