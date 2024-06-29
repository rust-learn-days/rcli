use std::str::FromStr;

use clap::Parser;

use super::verify_file_exists;

#[derive(Parser, Debug)]
pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: String,
    #[arg(long, default_value = "chacha20-poly1305", value_parser = parse_format)]
    pub format: TextEncryptFormat,
}

impl EncryptOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: String,
    #[arg(long, default_value = "chacha20-poly1305", value_parser = parse_format)]
    pub format: TextEncryptFormat,
}

impl DecryptOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct EncryptKeyOpts {
    #[arg(short, long, default_value = "encrypted-key.txt")]
    pub output: String,
    #[arg(long, default_value = "chacha20-poly1305", value_parser = parse_format)]
    pub format: TextEncryptFormat,
}

impl EncryptKeyOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextEncryptFormat {
    ChaCha20Poly1305,
}

fn parse_format(s: &str) -> Result<TextEncryptFormat, &'static str> {
    s.parse()
}

impl From<TextEncryptFormat> for &'static str {
    fn from(f: TextEncryptFormat) -> Self {
        match f {
            TextEncryptFormat::ChaCha20Poly1305 => "chacha20-poly1305",
        }
    }
}

impl FromStr for TextEncryptFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "chacha20-poly1305" => Ok(TextEncryptFormat::ChaCha20Poly1305),
            _ => Err("Invalid file format"),
        }
    }
}

impl std::fmt::Display for TextEncryptFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
