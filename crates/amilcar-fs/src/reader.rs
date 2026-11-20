use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

/// Open a file for buffered, sequential reading.
pub fn open_buffered(path: impl AsRef<Path>) -> io::Result<BufReader<File>> {
    File::open(path).map(BufReader::new)
}

/// Read a UTF-8 file only when its metadata size is within the supplied limit.
pub fn read_to_string_limited(path: impl AsRef<Path>, max_bytes: u64) -> io::Result<String> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path)?;
    if metadata.len() > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("file size {} exceeds limit {max_bytes}", metadata.len()),
        ));
    }
    let capacity = usize::try_from(metadata.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "file is too large for this platform",
        )
    })?;

    let mut contents = String::with_capacity(capacity);
    open_buffered(path)?.read_to_string(&mut contents)?;
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
