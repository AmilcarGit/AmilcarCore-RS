use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

/// Shared I/O buffer size used by streaming filesystem operations.
pub const BUFFER_SIZE: usize = 64 * 1024;

/// Open a file for buffered, sequential reading.
pub fn open_buffered(path: impl AsRef<Path>) -> io::Result<BufReader<File>> {
    File::open(path).map(|file| BufReader::with_capacity(BUFFER_SIZE, file))
}

/// Read a UTF-8 file within a strict byte limit, including if it grows while being read.
pub fn read_to_string_limited(path: impl AsRef<Path>, max_bytes: u64) -> io::Result<String> {
    let file = File::open(path)?;
    let size = file.metadata()?.len();
    if size > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("file size {size} exceeds limit {max_bytes}"),
        ));
    }
    let capacity = usize::try_from(size).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "file is too large for this platform",
        )
    })?;

    let mut contents = String::with_capacity(capacity);
    file.take(max_bytes.saturating_add(1))
        .read_to_string(&mut contents)?;
    if contents.len() as u64 > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("file grew beyond limit {max_bytes} while being read"),
        ));
    }
    Ok(contents)
}

/// Copy file contents using a fixed-size buffer, without loading the whole file.
pub fn copy_file_contents(
    source: impl AsRef<Path>,
    destination: &mut impl Write,
) -> io::Result<u64> {
    let mut reader = open_buffered(source)?;
    io::copy(&mut reader, destination)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_within_limit_and_rejects_oversized_file() {
        let path = std::env::temp_dir().join(format!("amilcar-reader-{}.txt", std::process::id()));
        std::fs::write(&path, "local data").unwrap();
        assert_eq!(read_to_string_limited(&path, 32).unwrap(), "local data");
        assert_eq!(
            read_to_string_limited(&path, 2).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        std::fs::remove_file(path).unwrap();
    }
}
