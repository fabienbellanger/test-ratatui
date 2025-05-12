use std::io;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    IO(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IO(err.to_string())
    }
}
