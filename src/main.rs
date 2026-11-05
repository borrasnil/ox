mod error;
use clap::Parser;
use error::OxError;

#[derive(Debug, Clone)]
enum Format {
    Binary,
    Hexadecimal,
    Octal,
    Decimal,
}

impl Format {
    fn from_str(s: &str) -> Result<Format, OxError> {
        match s {
            "0b" => Ok(Format::Binary),
            "0x" => Ok(Format::Hexadecimal),
            "0o" => Ok(Format::Octal),
            "0" => Ok(Format::Decimal),
            _ => Err(OxError::InvalidFormat),
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    value: String,

    #[arg(short, long, value_parser = Format::from_str, default_value = "0")]
    format: Format,
}

fn transform(value: &str, format: Format) -> Result<String, OxError> {
    let (digits, base) = if let Some(rest) = value.strip_prefix("0b") {
        (rest, 2)
    } else if let Some(rest) = value.strip_prefix("0x") {
        (rest, 16)
    } else if let Some(rest) = value.strip_prefix("0o") {
        (rest, 8)
    } else {
        (value, 10)
    };

    let num = i64::from_str_radix(digits, base).map_err(|_| OxError::ParsingFailed)?;

    match format {
        Format::Binary => Ok(format!("0b{:b}", num)),
        Format::Hexadecimal => Ok(format!("0x{:x}", num)),
        Format::Octal => Ok(format!("0o{:o}", num)),
        Format::Decimal => Ok(format!("{}", num)),
    }
}

fn main() {
    let args = Args::parse();

    let format = args.format;
    let value = args.value;

    match transform(&value, format) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}
