use std::str::FromStr;

use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;

use crate::{
    decrypt_message, encrypt_message, generate_encrypt_key, get_decrypt_key, process_from_input,
};

use super::verify_file_exists;

#[derive(Parser, Debug)]
pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(
        short, long, value_parser = verify_file_exists, default_value = "assets/encrypted-key.txt"
    )]
    pub key: String,
    #[arg(short, long, default_value = "assets/encrypted.txt")]
    pub output: String,
    #[arg(long, default_value = "chacha20-poly1305", value_parser = parse_format)]
    pub format: TextEncryptFormat,
}

impl EncryptOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        let key_bytes = process_from_input(&self.key)?;
        let base64 = BASE64_URL_SAFE_NO_PAD.decode(key_bytes)?;
        let key = get_decrypt_key(base64.as_slice());
        let message = process_from_input(&self.input)?;
        match encrypt_message(key, message.as_bytes()) {
            Ok(encrypted_message) => {
                let base64msg = BASE64_URL_SAFE_NO_PAD.encode(encrypted_message);
                std::fs::write(&self.output, base64msg)?;
                Ok(())
            }
            Err(_) => Err(anyhow::anyhow!("Encryption failed")),
        }
    }
}

#[derive(Parser, Debug)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "assets/encrypted.txt")]
    pub input: String,
    #[arg(
        short, long, value_parser = verify_file_exists, default_value = "assets/encrypted-key.txt"
    )]
    pub key: String,
    #[arg(long, default_value = "chacha20-poly1305", value_parser = parse_format)]
    pub format: TextEncryptFormat,
    #[arg(short, long, default_value = "assets/decrypted.txt")]
    pub output: String,
    #[arg(short, long, default_value = "false")]
    pub print: bool,
}

impl DecryptOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        let key_bytes = process_from_input(&self.key)?;
        let base64 = BASE64_URL_SAFE_NO_PAD.decode(key_bytes)?;
        let key = get_decrypt_key(base64.as_slice());
        let encrypted_message = process_from_input(&self.input)?;
        let base64msg = BASE64_URL_SAFE_NO_PAD.decode(encrypted_message)?;
        match decrypt_message(key, base64msg.as_slice()) {
            Ok(message) => {
                if self.print {
                    println!("{}", String::from_utf8_lossy(&message));
                }
                std::fs::write(&self.output, message)?;
                Ok(())
            }
            Err(_) => Err(anyhow::anyhow!("Decryption failed")),
        }
    }
}

#[derive(Parser, Debug)]
pub struct EncryptKeyOpts {
    #[arg(short, long, default_value = "assets/encrypted-key.txt")]
    pub output: String,
    #[arg(long, default_value = "chacha20-poly1305", value_parser = parse_format)]
    pub format: TextEncryptFormat,
}

impl EncryptKeyOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        //generate_encrypt_key(&self.output, self.format)?;
        match self.format {
            TextEncryptFormat::ChaCha20Poly1305 => {
                let key = generate_encrypt_key();
                let key_vec = key.as_slice().to_vec();
                let base64 = BASE64_URL_SAFE_NO_PAD.encode(key_vec);
                println!("encrypt key: {}", base64);
                std::fs::write(self.output, base64)?;
            }
        }
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
