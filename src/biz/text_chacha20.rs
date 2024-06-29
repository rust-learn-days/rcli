use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce};

#[allow(dead_code)]
fn generate_encrypt_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng)
}

#[allow(dead_code)]
fn encrypt_message(key: &Key, message: &[u8]) -> Result<Vec<u8>, ()> {
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = match cipher.encrypt(&nonce, message.as_ref()) {
        Ok(ciphertext) => ciphertext,
        Err(_) => return Err(()),
    };

    // Prepend nonce to ciphertext
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

#[allow(dead_code)]
fn decrypt_message(key: &Key, encrypted_message: &[u8]) -> Result<Vec<u8>, ()> {
    let cipher = ChaCha20Poly1305::new(key);

    // Extract nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_message.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(plaintext) => plaintext,
        Err(_) => return Err(()),
    };
    Ok(plaintext)
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
