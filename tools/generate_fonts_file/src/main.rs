mod font_entry;

use std::{fs::File, io::Read};

use clap::Parser;
use miette::{IntoDiagnostic, Result, WrapErr};

use crate::font_entry::FontEntry;

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

fn main() -> Result<()> {
    let args = Args::parse();

    let input_data = read_input_file(&args.file_in).wrap_err("Reading input data failed")?;
    println!("Size: {}", input_data.len());

    let mut leftover_data = input_data.as_slice();
    loop {
        let (s, font_entry) = FontEntry::try_consume(leftover_data)
            .wrap_err("Error while searching for next font entry")?;
        leftover_data = s;

        match font_entry {
            None => break,
            Some(font_entry) => {
                println!("{}", String::from_utf8(font_entry.name.to_vec()).unwrap())
            }
        }
    }

    Ok(())
}
