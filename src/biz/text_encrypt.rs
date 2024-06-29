use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce};

pub fn generate_encrypt_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng)
}

pub fn get_decrypt_key(key: &[u8]) -> &Key {
    let key = Key::from_slice(key);
    key
}
#[allow(clippy::needless_return)]
pub fn encrypt_message(key: &Key, message: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = match cipher.encrypt(&nonce, message.as_ref()) {
        Ok(ciphertext) => ciphertext,
        Err(e) => return Err(anyhow::anyhow!("Decryption failed: {}", e)),
    };

    // Prepend nonce to ciphertext
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

#[allow(clippy::needless_return)]
pub fn decrypt_message(key: &Key, encrypted_message: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let cipher = ChaCha20Poly1305::new(key);

    // Extract nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_message.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(plaintext) => Ok(plaintext),
        Err(e) => return Err(anyhow::anyhow!("Decryption failed: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let key = generate_encrypt_key();
        let message = b"plaintext message";

        let encrypted_message = encrypt_message(&key, message).unwrap();
        let decrypted_message = decrypt_message(&key, &encrypted_message).unwrap();

        assert_eq!(decrypted_message, message);
        println!("Encryption and decryption successful!");
    }
}
