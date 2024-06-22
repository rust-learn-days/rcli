use std::fmt;
use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli", about, version, author, long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Subcommand,
    #[arg(
        short,
        long,
        default_value = "false",
        long_help = "Print debug information"
    )]
    pub debug: bool,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[clap(name = "csv", about = "Convert CSV to File")]
    Csv(CsvOpts),
    #[clap(name = "genpass", about = "Generate password for random")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    Json,
    Yaml,
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file_exists)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, default_value = "json", value_parser = parse_file_format)]
    pub format: FileFormat,
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    #[arg(short = 'r', long = "show header", default_value_t = true)]
    pub header: bool,
}

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub upper: bool,
    #[arg(long, default_value_t = true)]
    pub lower: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_file_exists(path: &str) -> Result<String, String> {
    if std::path::Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err(format!("File not found: {}", path))
    }
}

fn parse_file_format(s: &str) -> Result<FileFormat, &'static str> {
    s.parse()
}

impl From<FileFormat> for &'static str {
    fn from(f: FileFormat) -> Self {
        match f {
            FileFormat::Json => "json",
            FileFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for FileFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(FileFormat::Json),
            "yaml" | "yml" => Ok(FileFormat::Yaml),
            _ => Err("Invalid file format"),
        }
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
