use std::path::{Path, PathBuf};

/// Metadata collected without loading file contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
}

/// Inspect a path without following a symbolic link.
pub fn inspect(path: &Path) -> std::io::Result<FileMetadata> {
    let metadata = std::fs::symlink_metadata(path)?;
    Ok(FileMetadata {
        path: path.to_path_buf(),
        size: metadata.len(),
        is_file: metadata.is_file(),
        is_directory: metadata.is_dir(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspect_reports_file_type() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let result = inspect(&path).expect("manifest should exist");
        assert!(result.is_file);
        assert!(!result.is_directory);
    }
}
