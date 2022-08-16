mod font_data;
mod font_entry;
mod u8_compression;

use std::{
    fs::File,
    io::{Read, Write},
};

use clap::Parser;
use miette::{IntoDiagnostic, Result, WrapErr};

use crate::{font_data::consume_font_data, font_entry::FontEntry};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path of the u8g2_fonts.c input file
    #[clap(value_parser)]
    file_in: String,

    /// The path of the rust output file
    #[clap(value_parser)]
    file_out: String,
}

fn read_input_file(file: &str) -> Result<Vec<u8>> {
    let mut data = Vec::new();

    File::open(&file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Unable to open '{}'", &file))?
        .read_to_end(&mut data)
        .into_diagnostic()
        .wrap_err("Error while reading file")?;

    Ok(data)
}

fn write_output_file(file: &str, data: &[u8]) -> Result<()> {
    File::create(file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Unable to open '{}'", &file))?
        .write_all(data)
        .into_diagnostic()
        .wrap_err("Error while writing file")
}

fn process_font_entry<'a>(
    font_entry: &FontEntry,
    out: &mut Vec<u8>,
    mut leftover_data: &'a [u8],
) -> Result<&'a [u8]> {
    println!(
        "{:>5} kB - {}",
        font_entry.expected_length / 1024 + 1,
        String::from_utf8(font_entry.name.to_vec()).unwrap(),
    );

    out.extend_from_slice(b"#[allow(non_camel_case_types)]\npub struct ");
    out.extend_from_slice(font_entry.name);
    out.extend_from_slice(b";\nimpl crate::font::Font for ");
    out.extend_from_slice(font_entry.name);
    out.extend_from_slice(b" {\n    const DATA: &'static [u8] = b\"");

    let (d, length) =
        consume_font_data(leftover_data, out).wrap_err("Unable to consume font data")?;
    leftover_data = d;

    miette::ensure!(
        length == font_entry.expected_length,
        "Expected to produce {} bytes, but produced {} bytes",
        font_entry.expected_length,
        length
    );

    out.extend_from_slice(b"\";\n}\n\n");

    Ok(leftover_data)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_data = read_input_file(&args.file_in).wrap_err("Reading input data failed")?;

    let mut out = Vec::new();

    let mut leftover_data = input_data.as_slice();
    loop {
        let (s, font_entry) = FontEntry::try_consume(leftover_data)
            .wrap_err("Error while searching for next font entry")?;
        leftover_data = s;

        match font_entry {
            None => break,
            Some(font_entry) => {
                // TODO: Remove the if
                if font_entry.name == b"u8g2_font_lubBI14_tf" {
                    leftover_data = process_font_entry(&font_entry, &mut out, &mut leftover_data)
                        .wrap_err("Error while processing font entry")?;
                }
            }
        }
    }

    write_output_file(&args.file_out, &out).wrap_err("Unable to write converted file")
}
