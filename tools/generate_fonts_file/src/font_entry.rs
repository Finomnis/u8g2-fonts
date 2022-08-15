use lazy_static::lazy_static;
use miette::{IntoDiagnostic, Result, WrapErr};
use regex::bytes::Regex;

pub struct FontEntry<'a> {
    pub name: &'a [u8],
    pub expected_length: usize,
}

lazy_static! {
    static ref FONT_REGEX: Regex =
        Regex::new(r#"const uint8_t (\w*)\[(\d*)\] U8G2_FONT_SECTION\("(\w*)"\) ="#,).unwrap();
}

impl<'a> FontEntry<'a> {
    pub fn try_consume(data: &'a [u8]) -> Result<(&'a [u8], Option<FontEntry<'a>>)> {
        let font_match = match FONT_REGEX.captures(data) {
            Some(f) => f,
            None => return Ok((data, None)),
        };

        let name = font_match.get(1).unwrap().as_bytes();
        let expected_length: usize =
            String::from_utf8(font_match.get(2).unwrap().as_bytes().to_vec())
                .into_diagnostic()
                .wrap_err("Unable to read font length")?
                .parse()
                .into_diagnostic()
                .wrap_err("Unable to read font length")?;
        let name2 = font_match.get(3).unwrap().as_bytes();

        assert!(name == name2);

        let font_entry = FontEntry {
            name,
            expected_length,
        };

        let leftover_data = &data[font_match.get(0).unwrap().range().end..];

        Ok((leftover_data, Some(font_entry)))
    }
}
