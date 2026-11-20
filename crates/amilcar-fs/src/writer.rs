use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

/// Explicit policy for writing a destination file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode {
    /// Create a new file and fail if the destination already exists.
    CreateNew,
    /// Truncate and replace an existing file.
    Overwrite,
}

/// Write bytes using an explicit destination policy.
pub fn write_bytes(path: impl AsRef<Path>, contents: &[u8], mode: WriteMode) -> io::Result<()> {
    let mut options = OpenOptions::new();
    options.write(true);
    match mode {
        WriteMode::CreateNew => {
            options.create_new(true);
        }
        WriteMode::Overwrite => {
            options.create(true).truncate(true);
        }
    }

    let mut file = options.open(path)?;
    file.write_all(contents)?;
    file.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_does_not_overwrite_existing_file() {
        let path = std::env::temp_dir().join(format!("amilcar-writer-{}.txt", std::process::id()));
        let _ = std::fs::remove_file(&path);
        write_bytes(&path, b"first", WriteMode::CreateNew).unwrap();
        assert!(write_bytes(&path, b"second", WriteMode::CreateNew).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"first");
        write_bytes(&path, b"replacement", WriteMode::Overwrite).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"replacement");
        std::fs::remove_file(path).unwrap();
    }
}
