use anyhow::{Context, Result, bail};
use clap::Parser;
use dialoguer::Input;
use lr_formatter_rs::formats::{json, lrb, sol, trk};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Parser)]
#[command(
    version = "1.1.0",
    author = "LRBSpec",
    about = "CLI for converting Line Rider file formats"
)]
struct Cli {
    /// Path of the file to convert
    input_file: String,
    /// Format to convert to (json, lrb, sol)
    output_format: String,
    /// Optional output file path
    output_file: Option<String>,
}

enum Format {
    Json,
    LRB,
    TRK,
    SOL(Option<u32>),
}

fn convert(input: Vec<u8>, from: Format, to: Format) -> Result<Vec<u8>> {
    let internal_format = match from {
        Format::Json => json::read(input)?,
        Format::LRB => lrb::read(input)?,
        Format::TRK => trk::read(input)?,
        Format::SOL(track_index) => sol::read(input, track_index)?,
    };

    let output_bytes = match to {
        Format::Json => json::write(&internal_format)?,
        Format::LRB => lrb::write(&internal_format)?,
        Format::SOL(_) => sol::write(&internal_format)?,
        _ => bail!("Unsupported to format. Must be one of: json, lrb, sol"),
    };

    Ok(output_bytes)
}

fn parse_format(format: &str, sol_index: Option<u32>) -> Result<Format> {
    match format.to_lowercase().as_str() {
        "json" => Ok(Format::Json),
        "lrb" => Ok(Format::LRB),
        "trk" => Ok(Format::TRK),
        "sol" => Ok(Format::SOL(sol_index)),
        _ => Err(anyhow::anyhow!(
            "Invalid format '{}'. Must be one of: json, lrb, trk, sol",
            format
        )),
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();

    let mut input_data = Vec::new();

    File::open(&args.input_file)
        .with_context(|| format!("Failed to open input file '{}'", &args.input_file))?
        .read_to_end(&mut input_data)
        .context("Failed to read input file")?;

    let input_path = Path::new(&args.input_file);
    let input_extension = input_path
        .extension()
        .and_then(|e| e.to_str())
        .context("Failed to parse file extension")?;
    let input_name = input_path
        .file_stem()
        .and_then(|e| e.to_str())
        .context("Failed to parse file name")?;

    let mut sol_index = None;

    if input_extension == "sol" {
        let max_index = sol::get_track_count(&input_data) - 1;
        if max_index > 0 {
            sol_index = Some(
                Input::new()
                    .with_prompt(format!(
                        "SOL detected, please enter track file index (0 - {})",
                        max_index
                    ))
                    .validate_with(|input: &u32| {
                        if (0..=max_index).contains(input) {
                            Ok(())
                        } else {
                            Err(format!(
                                "Track file index must be in range (0 - {})",
                                max_index
                            ))
                        }
                    })
                    .interact_text()
                    .unwrap_or(0),
            );
        }
    }

    let input_format =
        parse_format(input_extension, sol_index).context("Failed to parse input format")?;
    let output_format =
        parse_format(&args.output_format, None).context("Failed to parse output format")?;
    let output_extension = match output_format {
        Format::LRB => ".lrb",
        Format::SOL(_) => ".sol",
        Format::TRK => ".trk",
        Format::Json => ".track.json",
    };
    let output_file_name = &args
        .output_file
        .unwrap_or(input_name.to_string() + " (Converted)" + output_extension);
    let output_data =
        &convert(input_data, input_format, output_format).context("Conversion failed")?;

    File::create(output_file_name)
        .with_context(|| format!("Failed to create output file '{}'", output_file_name))?
        .write_all(output_data)
        .context("Failed to write output file")?;

    println!("Converted file saved to {}", output_file_name);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}
