use std::io::Read;

use anyhow::{Error, Result};
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{process_from_input, TextSignFormat};

trait TextSign {
    fn sign(&self, read: &mut dyn Read) -> Result<Vec<u8>, Error>;
}

trait TextVerify {
    fn verify(&self, read: impl Read, signature: Vec<u8>) -> Result<bool, Error>;
}

struct Blake3 {}

struct Ed25519Sign {
    key: SigningKey,
}

struct Ed25519Verify {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, read: &mut dyn Read) -> Result<Vec<u8>, Error> {
        let mut buffer = String::new();
        read.read_to_string(&mut buffer)?;
        let str = buffer.trim();
        let sign = blake3::hash(str.as_bytes()).as_bytes().to_vec();
        Ok(sign)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut read: impl Read, signature: Vec<u8>) -> Result<bool, Error> {
        let mut buffer = String::new();
        read.read_to_string(&mut buffer)?;
        let str = buffer.trim();
        let sign = blake3::hash(str.as_bytes()).as_bytes().to_vec();
        let signature = signature.as_slice();
        Ok(signature == sign)
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
    fn verify(&self, mut _read: impl Read, signature: Vec<u8>) -> Result<bool, Error> {
        let mut buffer = String::new();
        _read.read_to_string(&mut buffer)?;
        let str = buffer.trim();
        let sign_str = String::from_utf8(signature)?;
        let sign = Signature::from_bytes(sign_str.as_bytes().try_into()?);
        Ok(self.key.verify(str.as_bytes(), &sign).is_ok())
    }
}

pub fn sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let input_str = match process_from_input(input) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let key_str = match process_from_input(key) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let signature = match format {
        TextSignFormat::Blake3 => {
            let blake3 = Blake3 {};
            blake3.sign(&mut input_str.as_bytes())?
        }
        TextSignFormat::Ed25519 => {
            let key = SigningKey::from_bytes(key_str.as_bytes().try_into()?);
            let ed25519 = Ed25519Sign { key };
            ed25519.sign(&mut input_str.as_bytes())?
        }
    };
    let signature = BASE64_URL_SAFE_NO_PAD.encode(signature);
    println!("{}", signature);
    Ok(())
}

pub fn verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    signature: &str,
) -> anyhow::Result<()> {
    let input_str = match process_from_input(input) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let key_str = match process_from_input(key) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let signature = BASE64_URL_SAFE_NO_PAD.decode(signature.as_bytes())?;

    let signature = match format {
        TextSignFormat::Blake3 => {
            let blake3 = Blake3 {};
            blake3.verify(&mut input_str.as_bytes(), signature)?
        }
        TextSignFormat::Ed25519 => {
            let key = match VerifyingKey::from_bytes(key_str.as_bytes().try_into()?) {
                Ok(k) => k,
                Err(e) => return Err(e.into()),
            };
            let ed25519 = Ed25519Verify { key };
            ed25519.verify(&mut input_str.as_bytes(), signature)?
        }
    };
    if signature {
        println!("Signature is valid");
    } else {
        println!("Signature is invalid");
    }
    Ok(())
}
