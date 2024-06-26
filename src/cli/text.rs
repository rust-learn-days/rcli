use clap::Parser;

use super::verify_file_exists;

#[derive(Parser, Debug)]
pub struct TextOpts {
    #[clap(subcommand)]
    pub cmd: TextSubcommand,
}

#[derive(Parser, Debug)]
pub enum TextSubcommand {
    #[clap(name = "generate-key", about = "Generate key")]
    GenerateKey(GenerateKeyOpts),
    #[clap(name = "sign", about = "Sign text")]
    Sign(SignOpts),
    #[clap(name = "verify", about = "Verify text")]
    Verify(VerifyOpts),
}

#[derive(Parser, Debug)]
pub struct SignOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct VerifyOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub signature: String,
}

#[derive(Parser, Debug)]
pub struct GenerateKeyOpts {
    #[arg(short, long)]
    pub output: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(s: &str) -> Result<TextSignFormat, &'static str> {
    s.parse()
}

impl From<TextSignFormat> for &'static str {
    fn from(f: TextSignFormat) -> Self {
        match f {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl std::str::FromStr for TextSignFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err("Invalid file format"),
        }
    }
}

impl std::fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
