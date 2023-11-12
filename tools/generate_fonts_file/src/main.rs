mod font_data;
mod font_entry;
mod u8_compression;

use std::{
    fs::File,
    io::{Read, Write},
};

use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar};
use miette::{bail, IntoDiagnostic, Result, WrapErr};
use rayon::prelude::*;

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

    /// Doesn't write anything, but instead returns
    /// a non-zero exitcode if the generated code
    /// differs from the existing code
    #[clap(long)]
    check: bool,

    /// Hides the progress bar
    #[clap(long)]
    hide_progress: bool,
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

fn check_output_file(file: &str, data: &[u8]) -> Result<()> {
    let mut existing_data = Vec::new();

    File::open(&file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Unable to open '{}'", &file))?
        .read_to_end(&mut existing_data)
        .into_diagnostic()
        .wrap_err("Error while reading file")?;

    if data != existing_data {
        bail!("The generated code differs from the existing code!");
    }

    Ok(())
}

fn process_font_entry<'a>(font_entry: FontEntry<'a>, out: &mut Vec<u8>) -> Result<()> {
    out.extend_from_slice(b"\npub struct ");
    out.extend_from_slice(font_entry.name.as_bytes());
    out.extend_from_slice(b";\nimpl Font for ");
    out.extend_from_slice(font_entry.name.as_bytes());
    out.extend_from_slice(b" {\n    const DATA: &'static [u8] = b\"");

    let length = consume_font_data(font_entry.data, out).wrap_err("Unable to consume font data")?;

    miette::ensure!(
        length == font_entry.expected_length,
        "Expected to produce {} bytes, but produced {} bytes",
        font_entry.expected_length,
        length
    );

    out.extend_from_slice(b"\";\n}\n");

    Ok(())
}

fn pre_parse_fonts(mut data: &[u8]) -> Result<Vec<FontEntry>> {
    let mut result = Vec::new();
    loop {
        let (leftover, entry) = FontEntry::try_consume(data)?;
        data = leftover;

        match entry {
            Some(entry) => {
                result.push(entry);
            }
            None => break,
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_data = read_input_file(&args.file_in).wrap_err("Reading input data failed")?;

    let mut out = Vec::new();

    out.extend_from_slice(b"use crate::Font;\n");

    let fonts = pre_parse_fonts(&input_data)
        .wrap_err("Unable to parse fonts file!")?
        .into_par_iter();
    println!("Found {} fonts.", fonts.len());

    let mut print_msg: Box<dyn Fn(String) + Sync + Send> = Box::new(|s| println!("{}", s));
    let fonts = if args.hide_progress {
        fonts.progress_with(ProgressBar::hidden())
    } else {
        let fonts = fonts.progress();
        let progress_bar = fonts.progress.clone();
        print_msg = Box::new(move |s| progress_bar.println(s));
        fonts
    };

    let font_data = fonts
        .map(|font_entry| {
            print_msg(format!(
                "{:>5} kB - {}",
                font_entry.expected_length / 1024 + 1,
                font_entry.name,
            ));

            let mut font_out = Vec::new();
            process_font_entry(font_entry, &mut font_out)
                .wrap_err("Error while processing font entry")
                .unwrap();
            font_out
        })
        .flatten();

    out = out.into_par_iter().chain(font_data).collect();

    if args.check {
        check_output_file(&args.file_out, &out)
            .wrap_err("Verifying integrity of generated file failed")
    } else {
        write_output_file(&args.file_out, &out).wrap_err("Unable to write converted file")
    }
}
