use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxideError {
    #[error("wrong password")]
    WrongPassword,

    #[error("failed to read file: {0}")]
    ReadFailed(std::io::Error),

    #[error("failed to write file: {0}")]
    WriteFailed(std::io::Error)
}