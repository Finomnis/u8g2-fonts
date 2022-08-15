use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use regex::bytes;

struct EscapedCStr<'a>(&'a [u8]);

impl<'a> EscapedCStr<'a> {
    fn peek_next(&self) -> Option<u8> {
        self.0.get(0).cloned()
    }
    fn consume_one(&mut self) {
        self.0 = &self.0[1..];
    }
}

fn convert_escaped_cstr_to_u8(escaped: &[u8], out: &mut Vec<u8>) -> usize {
    let mut escaped = EscapedCStr(escaped);

    let mut num_produced = 0;

    fn read_next_byte(escaped: &mut EscapedCStr) -> Option<u8> {
        let result = Some(match escaped.peek_next()? {
            b'\\' => {
                escaped.consume_one();

                let next = escaped
                    .peek_next()
                    .expect("Found escape character with nothing after it");
                assert!(next >= b'0' && next < b'8');
                escaped.consume_one();

                let next = next - b'0';

                let mut c = next;

                if let Some(next) = escaped.peek_next() {
                    if next >= b'0' && next < b'8' {
                        let next = next - b'0';
                        c = 8 * c + next;
                        escaped.consume_one();
                        if let Some(next) = escaped.peek_next() {
                            if next >= b'0' && next < b'8' {
                                let next = next - b'0';
                                c = 8 * c + next;
                                escaped.consume_one();
                            }
                        }
                    }
                }

                c
            }
            c => {
                escaped.consume_one();
                c
            }
        });
        result
    }

    while let Some(next) = read_next_byte(&mut escaped) {
        out.extend_from_slice(format!("\\x{:02x}", next).as_bytes());
        num_produced += 1;
    }

    num_produced
}

fn parse_raw_font_data(mut raw_data: &[u8], out: &mut Vec<u8>) -> usize {
    let mut num_produced = 0;
    loop {
        match raw_data.get(0) {
            Some(b'\r') | Some(b'\n') | Some(b' ') | Some(b'\t') => raw_data = &raw_data[1..],
            Some(b'"') => {
                let pos_closing = raw_data
                    .iter()
                    .enumerate()
                    .skip(1)
                    .find(|(_, &v)| v == b'"')
                    .expect("Unable to find closing quote!")
                    .0;

                let data_sentence = &raw_data[1..pos_closing];
                raw_data = &raw_data[(pos_closing + 1)..];

                num_produced += convert_escaped_cstr_to_u8(data_sentence, out);
            }
            Some(_) => panic!("Unexpected: {}", String::from_utf8_lossy(raw_data)),
            None => break,
        }
    }

    // Add zero-termination (C strings have a hidden '\0' at the end)
    out.extend_from_slice(b"\\0");

    num_produced + 1
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let fonts_path_in = "u8g2/csrc/u8g2_fonts.c";
    let fonts_path_out = out_path.join("generated_font_data.rs");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={fonts_path_in}");

    // Read fonts data c-file
    let fonts_raw_input = {
        let mut s = Vec::new();
        File::open(fonts_path_in)
            .expect("Unable to open fonts input file!")
            .read_to_end(&mut s)
            .expect("Unable to read fonts input file!");
        s
    };

    // Parse fonts data
    let font_regex = bytes::Regex::new(
        r#"const uint8_t (\w*)\[(\d*)\] U8G2_FONT_SECTION\("(\w*)"\) =((?:\s*"[^"]*")*);"#,
    )
    .unwrap();

    let mut fonts_output = Vec::new();

    for font_match in font_regex.captures_iter(&fonts_raw_input) {
        let name = font_match.get(1).unwrap().as_bytes();
        let expected_length: usize =
            String::from_utf8(font_match.get(2).unwrap().as_bytes().to_vec())
                .unwrap()
                .parse()
                .unwrap();
        let name2 = font_match.get(3).unwrap().as_bytes();
        let raw_data = font_match.get(4).unwrap().as_bytes();

        assert!(name == name2);

        fonts_output.extend_from_slice(b"#[allow(non_camel_case_types)]\npub struct ");
        fonts_output.extend_from_slice(name);
        fonts_output.extend_from_slice(b";\nimpl crate::font::Font for ");
        fonts_output.extend_from_slice(name);
        fonts_output.extend_from_slice(b"{\n    const DATA: &'static [u8] = b\"");
        let num_produced = parse_raw_font_data(raw_data, &mut fonts_output);
        assert_eq!(num_produced, expected_length);
        fonts_output.extend_from_slice(b"\";\n}\n\n");

        break;
    }

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    File::create(fonts_path_out)
        .expect("Unable to create font data file!")
        .write_all(&fonts_output)
        .unwrap();
}
