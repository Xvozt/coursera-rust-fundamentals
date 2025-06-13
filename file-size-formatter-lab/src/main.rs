use core::f64;
use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

enum SizeUnit {
    Bytes,
    Kylobytes,
    Megabytes,
    Gigabytes,
}

impl Sizes {
    fn from_str(input: &str) -> Result<Self, String> {
        let (size, unit) = parse_size(input)?;
        Ok(from_size_and_unit(size, unit))
    }
}

fn parse_size(input: &str) -> Result<(f64, SizeUnit), String> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        return Err("Input should be in format like '24 mb'".to_string());
    }

    let size = match parts[0].parse::<f64>() {
        Ok(v) => v,
        Err(_) => return Err("Failed to parse the numeric value".to_string()),
    };

    let unit = match parts[1].to_lowercase().as_str() {
        "b" | "bytes" | "byte" => SizeUnit::Bytes,
        "kb" | "kilobytes" | "kilobyte" | "k" => SizeUnit::Kylobytes,
        "mb" | "megabytes" | "megabyte" | "m" => SizeUnit::Megabytes,
        "gb" | "gigabytes" | "gigabyte" | "g" => SizeUnit::Gigabytes,
        unknown => return Err(format!("Unknown unit: {}. Use b, kb, mb or gb", unknown)),
    };

    Ok((size, unit))
}

fn from_size_and_unit(value: f64, unit: SizeUnit) -> Sizes {
    let bytes = match unit {
        SizeUnit::Bytes => value,
        SizeUnit::Kylobytes => value * 1000.0,
        SizeUnit::Megabytes => value * 1_000_000.0,
        SizeUnit::Gigabytes => value * 1_000_000_000.0,
    };

    let kilobytes = bytes / 1000.0;
    let megabytes = kilobytes / 1000.0;
    let gigabytes = megabytes / 1000.0;

    Sizes {
        bytes: format!("{} bytes", bytes as f64),
        kilobytes: format!("{} kilobytes", kilobytes as f64),
        megabytes: format!("{} megabytes", megabytes as f64),
        gigabytes: format!("{} gigabytes", gigabytes as f64),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run \"24 mb\"");
        std::process::exit(-1);
    }

    match Sizes::from_str(&args[1]) {
        Ok(sizes) => println!("{:?}", sizes),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(-1);
        }
    }
}
