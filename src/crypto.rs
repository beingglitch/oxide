use crate::error::OxideError;
use aes_gcm::{Aes256Gcm, Nonce, KeyInit};
use aead::Aead;
use rand::rngs::OsRng;
use rand::RngCore;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha256;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12; // 96-bit recommended for AES-GCM
const KEY_LEN: usize = 32;   // 256-bit key
const PBKDF2_ITERS: u32 = 100_000; // tune for your environment

pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>, OxideError> {
    let salt = generate_salt();

    let key = derive_key(password, &salt);

    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| OxideError::EncryptionError(format!("invalid key: {}", e)))?;

    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| OxideError::EncryptionError(format!("AES-GCM encrypt failed: {}", e)))?;


    let mut out = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);

    Ok(out)
}

pub fn decrypt(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, OxideError> {
    if encrypted_data.len() < SALT_LEN + NONCE_LEN + 1 {
        return Err(OxideError::DecryptionError);
    }

    let salt = &encrypted_data[..SALT_LEN];
    let nonce_bytes = &encrypted_data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &encrypted_data[SALT_LEN + NONCE_LEN..];

    let key = derive_key(password, salt);

    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| OxideError::EncryptionError(format!("invalid key: {}", e)))?;
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| OxideError::DecryptionError)?;

    Ok(plaintext)
}

fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, PBKDF2_ITERS, &mut key);
    key
}

fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    salt
}

fn generate_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    nonce
}