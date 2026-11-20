/// Errors returned by the public engine API.
#[derive(Debug, thiserror::Error)]
pub enum AmilcarError {
    #[error("filesystem operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("file exceeds configured size limit ({size} bytes > {limit} bytes)")]
    FileTooLarge { size: u64, limit: u64 },
    #[error("data processing failed: {0}")]
    Data(String),
}
/// Result type used by public APIs.
pub type Result<T> = std::result::Result<T, AmilcarError>;
