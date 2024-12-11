mod font_data;
mod font_entry;

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar};
use miette::{IntoDiagnostic, Result, WrapErr};
use rayon::prelude::*;

use crate::{font_data::consume_font_data, font_entry::FontEntry};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path of the u8g2_fonts.c input file
    #[arg()]
    file_in: String,

    /// The path of the rust output directory
    #[arg()]
    dir_out: PathBuf,

    /// Hides the progress bar
    #[arg(long)]
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

fn write_file(file: &Path, data: &[u8]) -> Result<()> {
    File::create(file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Unable to open '{:?}'", &file))?
        .write_all(data)
        .into_diagnostic()
        .wrap_err("Error while writing file")
}

fn process_font_entry<'a>(font_entry: FontEntry<'a>) -> Result<Box<[u8]>> {
    let mut font_data = vec![];

    let length = consume_font_data(font_entry.data, &mut font_data)
        .wrap_err("Unable to consume font data")?;

    miette::ensure!(
        length == font_entry.expected_length,
        "Expected to produce {} bytes, but produced {} bytes",
        font_entry.expected_length,
        length
    );

    Ok(font_data.into_boxed_slice())
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

    std::fs::remove_dir_all(&args.dir_out)
        .into_diagnostic()
        .wrap_err("Unable to remove output directory!")?;
    std::fs::create_dir(&args.dir_out)
        .into_diagnostic()
        .wrap_err("Unable to create output directory!")?;

    let mut print_msg: Box<dyn Fn(String) + Sync + Send> = Box::new(|s| println!("{}", s));
    let fonts = if args.hide_progress {
        fonts.progress_with(ProgressBar::hidden())
    } else {
        let fonts = fonts.progress();
        let progress_bar = fonts.progress.clone();
        print_msg = Box::new(move |s| progress_bar.println(s));
        fonts
    };

    let font_names = fonts
        .map(|font_entry| {
            print_msg(format!(
                "{:>5} kB - {}",
                font_entry.expected_length / 1024 + 1,
                font_entry.name,
            ));

            let name = font_entry.name;

            let font_data = process_font_entry(font_entry)
                .wrap_err("Error while processing font entry")
                .unwrap();

            write_file(&args.dir_out.join(format!("{name}.u8g2font")), &font_data)
                .wrap_err(format!("Failed to write font file '{name}'"))
                .unwrap();

            name
        })
        .collect::<Vec<_>>();

    let mut fonts_file = File::create(args.dir_out.join("mod.rs"))
        .into_diagnostic()
        .wrap_err_with(|| format!("Unable to open '{:?}'", args.dir_out.join("mod.rs")))?;

    write!(fonts_file, "crate::font::font_definitions!(\n")
        .into_diagnostic()
        .wrap_err("Error while writing mod.rs!")?;

    for name in font_names {
        write!(fonts_file, "    {name},\n")
            .into_diagnostic()
            .wrap_err("Error while writing mod.rs!")?;
    }

    write!(fonts_file, ");\n")
        .into_diagnostic()
        .wrap_err("Error while writing file")?;

    Ok(())
}
