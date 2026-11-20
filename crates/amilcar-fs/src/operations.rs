use std::fs::{self, OpenOptions};
use std::io::{self, BufReader, Write};
use std::path::Path;

/// Copy a file using an explicit overwrite policy. The default policy is CreateNew.
pub fn copy_file(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    overwrite: bool,
) -> io::Result<u64> {
    let source = source.as_ref();
    let destination = destination.as_ref();
    if source == destination {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "source and destination are the same path",
        ));
    }
    let metadata = fs::symlink_metadata(source)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "source must be a regular file, not a symbolic link",
        ));
    }
    if let Ok(destination_metadata) = fs::symlink_metadata(destination) {
        if destination_metadata.file_type().is_symlink() || destination_metadata.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "destination must not be a symbolic link or directory",
            ));
        }
    }
    let mut input = BufReader::new(fs::File::open(source)?);
    let mut options = OpenOptions::new();
    options.write(true);
    if overwrite {
        options.create(true).truncate(true);
    } else {
        options.create_new(true);
    }
    let mut output = options.open(destination)?;
    let copied = io::copy(&mut input, &mut output)?;
    output.flush()?;
    Ok(copied)
}

/// Move a file or directory when the destination does not exist.
pub fn move_path(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    let source = source.as_ref();
    let destination = destination.as_ref();
    if source == destination {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "source and destination are the same path",
        ));
    }
    if fs::symlink_metadata(destination).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "destination already exists",
        ));
    }
    fs::rename(source, destination)
}

/// Rename a path in its current parent directory without replacing an existing entry.
pub fn rename_path(
    source: impl AsRef<Path>,
    new_name: impl AsRef<std::ffi::OsStr>,
) -> io::Result<()> {
    let source = source.as_ref();
    let name = new_name.as_ref();
    let destination = source.parent().unwrap_or_else(|| Path::new(".")).join(name);
    if name.is_empty()
        || Path::new(name).file_name() != Some(name)
        || name == std::ffi::OsStr::new(".")
        || name == std::ffi::OsStr::new("..")
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "new name must be a single path component",
        ));
    }
    move_path(source, destination)
}

/// Create a directory and its missing parents.
pub fn create_directory(path: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Remove one regular file. Symbolic links are rejected.
pub fn remove_file(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "target must be a regular file",
        ));
    }
    fs::remove_file(path)
}

/// Remove an empty directory only; this never recursively deletes contents.
pub fn remove_empty_directory(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "target must be a directory, not a symbolic link",
        ));
    }
    fs::remove_dir(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "amilcar-ops-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn copy_is_create_new_by_default_and_can_explicitly_overwrite() {
        let dir = temp_dir();
        let src = dir.join("src");
        let dst = dir.join("dst");
        fs::write(&src, b"first").unwrap();
        assert_eq!(copy_file(&src, &dst, false).unwrap(), 5);
        assert!(copy_file(&src, &dst, false).is_err());
        fs::write(&src, b"second").unwrap();
        copy_file(&src, &dst, true).unwrap();
        assert_eq!(fs::read(&dst).unwrap(), b"second");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn rename_move_and_safe_removal_work() {
        let dir = temp_dir();
        let source = dir.join("old.txt");
        fs::write(&source, b"x").unwrap();
        rename_path(&source, "new.txt").unwrap();
        let moved = dir.join("nested.txt");
        move_path(dir.join("new.txt"), &moved).unwrap();
        remove_file(&moved).unwrap();
        let empty = dir.join("empty");
        create_directory(&empty).unwrap();
        remove_empty_directory(&empty).unwrap();
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn remove_empty_directory_refuses_non_empty_directory() {
        let dir = temp_dir();
        fs::write(dir.join("keep"), b"data").unwrap();
        assert!(remove_empty_directory(&dir).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
