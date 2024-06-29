use std::fs;

use clap::Parser;
use colored::Colorize;

use crate::cli::text_encrypt::{DecryptOpts, EncryptOpts};
use crate::{generate_key, process_from_input, EncryptKeyOpts};

use super::verify_file;

#[derive(Parser, Debug)]
pub struct TextOpts {
    #[clap(subcommand)]
    pub cmd: TextSubcommand,
}

impl TextOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        match self.cmd {
            TextSubcommand::GenerateKey(opts) => opts.execute(),
            TextSubcommand::Sign(opts) => opts.execute(),
            TextSubcommand::Verify(opts) => opts.execute(),
            TextSubcommand::Encrypt(opts) => opts.execute(),
            TextSubcommand::Decrypt(opts) => opts.execute(),
            TextSubcommand::EncryptKey(opts) => opts.execute(),
        }
    }
}

#[derive(Parser, Debug)]
pub enum TextSubcommand {
    #[clap(name = "gen-key", about = "Generate key")]
    GenerateKey(GenerateKeyOpts),
    #[clap(name = "sign", about = "Sign text")]
    Sign(SignOpts),
    #[clap(name = "verify", about = "Verify text")]
    Verify(VerifyOpts),
    #[clap(name = "encrypt", about = "Encrypt text")]
    Encrypt(EncryptOpts),
    #[clap(name = "decrypt", about = "Decrypt text")]
    Decrypt(DecryptOpts),
    #[clap(name = "encrypt-key", about = "Encrypt key")]
    EncryptKey(EncryptKeyOpts),
}

#[derive(Parser, Debug)]
pub struct SignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

impl SignOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        let input: String = process_from_input(&self.input).unwrap();
        let key = process_from_input(&self.key).unwrap();
        match crate::sign(&mut input.as_bytes(), key.as_str(), self.format) {
            Ok(signature) => {
                println!("{} {}", "Signature: ".blue(), signature.blue());
                Ok(())
            }
            Err(e) => {
                eprintln!("{} {}", "Error: ".red(), e);
                Err(e)
            }
        }
    }
}

#[derive(Parser, Debug)]
pub struct VerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub signature: String,
}

impl VerifyOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        let input: String = process_from_input(&self.input).unwrap();
        let key = process_from_input(&self.key).unwrap();
        match crate::verify(
            &mut input.as_bytes(),
            key.as_str(),
            self.format,
            &self.signature,
        ) {
            Ok(ok) => {
                println!("{} {:?}", "Verify: ".blue(), ok);
                Ok(())
            }
            Err(e) => {
                eprintln!("{} {}", "Error: ".red(), e);
                Err(e)
            }
        }
    }
}

#[derive(Parser, Debug)]
pub struct GenerateKeyOpts {
    #[arg(short, long, default_value = "assets/blake3.txt")]
    pub output: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

impl GenerateKeyOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        match generate_key(self.format) {
            Ok(key) => {
                println!("Generated Key: {}", String::from_utf8(key.clone()).unwrap());
                fs::write(self.output, key.as_slice()).unwrap();
                Ok(())
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(e)
            }
        }
    }
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
