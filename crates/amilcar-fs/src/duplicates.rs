use crate::{find_files, sha256_file, DirectoryOptions, FileFilter};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Groups of files with identical SHA-256 digests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateGroup {
    pub sha256: String,
    pub files: Vec<PathBuf>,
}

/// Find duplicate regular files under a directory. Traversal is bounded by max_entries.
/// Files are grouped by size first so unique-sized files are never hashed.
pub fn find_duplicates(
    root: impl AsRef<Path>,
    max_entries: usize,
) -> io::Result<Vec<DuplicateGroup>> {
    let files = find_files(
        root,
        &FileFilter::default(),
        &DirectoryOptions { max_entries },
    )?;
    let mut by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for path in files {
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            continue;
        }
        by_size.entry(metadata.len()).or_default().push(path);
    }

    let mut by_digest: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for same_size_files in by_size.into_values().filter(|group| group.len() > 1) {
        for path in same_size_files {
            let digest = sha256_file(&path)?;
            by_digest.entry(digest).or_default().push(path);
        }
    }

    let mut groups: Vec<_> = by_digest
        .into_iter()
        .filter_map(|(sha256, mut files)| {
            if files.len() < 2 {
                None
            } else {
                files.sort();
                Some(DuplicateGroup { sha256, files })
            }
        })
        .collect();
    groups.sort_by(|a, b| a.sha256.cmp(&b.sha256));
    Ok(groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_files_with_equal_content() {
        let root = std::env::temp_dir().join(format!("amilcar-dupes-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("a.txt"), b"same").unwrap();
        std::fs::write(root.join("b.txt"), b"same").unwrap();
        std::fs::write(root.join("c.txt"), b"different").unwrap();
        let groups = find_duplicates(&root, 100).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].files.len(), 2);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn same_size_but_different_content_is_not_duplicate() {
        let root = std::env::temp_dir().join(format!("amilcar-dupes-size-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("a.txt"), b"same").unwrap();
        std::fs::write(root.join("b.txt"), b"else").unwrap();
        let groups = find_duplicates(&root, 100).unwrap();
        assert!(groups.is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }
}
