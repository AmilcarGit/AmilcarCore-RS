use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Filters supported by the directory search helper.
#[derive(Debug, Clone, Default)]
pub struct FileFilter {
    /// Match a file extension without a leading dot, e.g. "json".
    pub extension: Option<String>,
    /// Match this case-insensitive substring in the file name.
    pub name_contains: Option<String>,
}

/// Bounds and traversal behavior for directory searches.
#[derive(Debug, Clone)]
pub struct DirectoryOptions {
    /// Maximum number of filesystem entries visited.
    pub max_entries: usize,
}

impl Default for DirectoryOptions {
    fn default() -> Self {
        Self { max_entries: 100_000 }
    }
}

/// Find regular files recursively. Symbolic links are not followed.
pub fn find_files(
    root: impl AsRef<Path>,
    filter: &FileFilter,
    options: &DirectoryOptions,
) -> io::Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let mut pending = vec![root.to_path_buf()];
    let mut matches = Vec::new();
    let mut visited = 0_usize;

    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory)? {
            visited += 1;
            if visited > options.max_entries {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("directory search exceeded {} entries", options.max_entries),
                ));
            }

            let entry = entry?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)?;
            if metadata.file_type().is_symlink() {
                continue;
            }
            if metadata.is_dir() {
                pending.push(path);
            } else if metadata.is_file() && matches_filter(&path, filter) {
                matches.push(path);
            }
        }
    }

    matches.sort();
    Ok(matches)
}

fn matches_filter(path: &Path, filter: &FileFilter) -> bool {
    let name_matches = filter.name_contains.as_ref().is_none_or(|needle| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.to_lowercase().contains(&needle.to_lowercase()))
    });
    let extension_matches = filter.extension.as_ref().is_none_or(|extension| {
        path.extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(extension.trim_start_matches('.')))
    });
    name_matches && extension_matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_by_extension_recursively() {
        let root = std::env::temp_dir().join(format!("amilcar-find-{}", std::process::id()));
        let nested = root.join("nested");
        fs::create_dir_all(&nested).unwrap();
        fs::write(nested.join("sample.JSON"), "{}").unwrap();
        fs::write(root.join("notes.txt"), "text").unwrap();
        let found = find_files(
            &root,
            &FileFilter { extension: Some("json".into()), name_contains: None },
            &DirectoryOptions::default(),
        ).unwrap();
        assert_eq!(found.len(), 1);
        assert!(found[0].ends_with("sample.JSON"));
        fs::remove_dir_all(root).unwrap();
    }
}
