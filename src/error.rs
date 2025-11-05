use thiserror::Error;

#[derive(Error, Debug)] 
pub enum OxideError {
    #[error("Failed to read file: {0}")]
    FileReadError(String),

    #[error("Failed to write file: {0}")]
    FileWriteError(String),

    #[error("Encryption failed: {0}")]
    EncryptionError(String),

    #[error("Decryption failed (wrong password or corrupted file)")]
    DecryptionError,

    #[error("Invalid encrypted file format")]
    InvalidFormatError
}