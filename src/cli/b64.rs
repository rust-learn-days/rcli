use clap::Parser;
use colored::Colorize;

use crate::encode;

use super::verify_file;

#[derive(Parser, Debug)]
pub struct Base64Opts {
    #[clap(subcommand)]
    pub cmd: Base64Subcommand,
}

impl Base64Opts {
    pub fn execute(self) -> anyhow::Result<()> {
        match self.cmd {
            Base64Subcommand::Encode(encode_opts) => encode_opts.execute(),
            Base64Subcommand::Decode(decode_opts) => decode_opts.execute(),
        }
    }
}

#[derive(Parser, Debug)]
pub enum Base64Subcommand {
    #[clap(name = "encode", about = "Encode base64")]
    Encode(EncodeOpts),
    #[clap(name = "decode", about = "Decode base64")]
    Decode(DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "standard", value_parser = parse_format)]
    pub format: Format,
}

impl EncodeOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        if let Err(e) = encode(&self.input, self.format) {
            eprintln!("{} {}", "Error: ".red(), e);
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "standard", value_parser = parse_format)]
    pub format: Format,
}

impl DecodeOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        if let Err(e) = crate::decode(&self.input, self.format) {
            eprintln!("{} {}", "Error: ".red(), e);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Standard,
    UrlSafe,
}

fn parse_format(s: &str) -> Result<Format, &'static str> {
    s.parse()
}

impl From<Format> for &'static str {
    fn from(f: Format) -> Self {
        match f {
            Format::Standard => "standard",
            Format::UrlSafe => "url_safe",
        }
    }
}

impl std::str::FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Format::Standard),
            "url_safe" => Ok(Format::UrlSafe),
            _ => Err("Invalid format"),
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
