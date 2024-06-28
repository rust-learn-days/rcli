use std::io::Read;

use anyhow::{Error, Result};
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use blake3::KEY_LEN;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{gen_pass, TextSignFormat};

pub trait TextSign {
    fn sign(&self, read: &mut dyn Read) -> Result<Vec<u8>, Error>;
}

pub trait TextVerify {
    fn verify(&self, read: &mut dyn Read, signature: Vec<u8>) -> Result<bool, Error>;
}

pub struct Blake3 {
    key: [u8; KEY_LEN],
}

pub struct Ed25519Sign {
    key: SigningKey,
}

pub struct Ed25519Verify {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, read: &mut dyn Read) -> Result<Vec<u8>, Error> {
        let mut buffer = String::new();
        read.read_to_string(&mut buffer)?;
        let str = buffer.trim();
        let sign = blake3::keyed_hash(&self.key, str.as_bytes())
            .as_bytes()
            .to_vec();
        Ok(sign)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, signature: Vec<u8>) -> Result<bool, Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sign = blake3::keyed_hash(&self.key, &buf);
        let signature = signature.as_slice();
        Ok(signature == sign.as_bytes())
    }
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    #[allow(dead_code)]
    fn generate_key() -> Result<Vec<u8>> {
        let key = gen_pass(32, true, true, true, true)?;
        Ok(key.as_bytes().to_vec())
    }
}

impl TextSign for Ed25519Sign {
    fn sign(&self, _read: &mut dyn Read) -> Result<Vec<u8>, Error> {
        let mut buffer = String::new();
        _read.read_to_string(&mut buffer)?;
        let str = buffer.trim();
        let sign = self.key.sign(str.as_bytes());
        Ok(sign.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verify {
    fn verify(&self, _read: &mut dyn Read, signature: Vec<u8>) -> Result<bool, Error> {
        let mut buffer = String::new();
        _read.read_to_string(&mut buffer)?;
        let str = buffer.trim();
        let sign_str = String::from_utf8(signature)?;
        let sign = Signature::from_bytes(sign_str.as_bytes().try_into()?);
        Ok(self.key.verify(str.as_bytes(), &sign).is_ok())
    }
}

pub fn sign(input: &mut dyn Read, key: &str, format: TextSignFormat) -> Result<String> {
    let signature: Box<dyn TextSign> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => {
            todo!()
        }
    };
    let signature = signature.sign(input)?;
    let signature_base64 = BASE64_URL_SAFE_NO_PAD.encode(signature);
    Ok(signature_base64)
}

pub fn verify(
    input: &mut dyn Read,
    key: &str,
    format: TextSignFormat,
    signature: &str,
) -> Result<bool> {
    let signature = BASE64_URL_SAFE_NO_PAD.decode(signature.as_bytes())?;
    let verifier: Box<dyn TextVerify> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => {
            todo!()
        }
    };
    let signature = verifier.verify(input, signature)?;
    Ok(signature)
}

pub fn generate_key(format: TextSignFormat) -> Result<Vec<u8>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate_key(),
        TextSignFormat::Ed25519 => {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let mut input = "hello".as_bytes();
        let key = gen_pass(32, true, true, true, true).unwrap();
        let format = TextSignFormat::Blake3;
        let out = match sign(&mut input, &key, format) {
            Ok(sign) => {
                println!("Sign: {}", sign);
                Ok(sign)
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(e)
            }
        };
        let mut new_input = "hello".as_bytes();
        let ok = verify(&mut new_input, &key, format, out.unwrap().as_str());
        println!("Verify: {:?}", ok);
        assert!(ok.is_ok());
    }
}
