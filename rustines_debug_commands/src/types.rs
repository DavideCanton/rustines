use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustinesDebugError {
    #[error("Failed to open file: {0}")]
    ReadFileError(#[from] io::Error),
    #[error("Failed to load ROM: {0}")]
    FileFormatError(String),
}
