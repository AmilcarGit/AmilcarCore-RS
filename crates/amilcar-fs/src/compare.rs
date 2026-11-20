use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Result of comparing two files byte by byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileComparison {
    pub identical: bool,
    pub first_difference: Option<u64>,
    pub left_size: u64,
    pub right_size: u64,
}

fn read_chunk(reader: &mut impl Read, buffer: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;
    while filled < buffer.len() {
        let count = reader.read(&mut buffer[filled..])?;
        if count == 0 {
            break;
        }
        filled += count;
    }
    Ok(filled)
}

/// Compare two files in bounded memory and report the first differing byte offset.
pub fn compare_files(
    left: impl AsRef<Path>,
    right: impl AsRef<Path>,
) -> io::Result<FileComparison> {
    let left = left.as_ref();
    let right = right.as_ref();
    let left_size = std::fs::metadata(left)?.len();
    let right_size = std::fs::metadata(right)?.len();
    let mut a = File::open(left)?;
    let mut b = File::open(right)?;
    let mut a_buf = [0_u8; crate::reader::BUFFER_SIZE];
    let mut b_buf = [0_u8; crate::reader::BUFFER_SIZE];
    let mut offset = 0_u64;

    loop {
        let a_read = read_chunk(&mut a, &mut a_buf)?;
        let b_read = read_chunk(&mut b, &mut b_buf)?;
        let shared = a_read.min(b_read);

        if let Some(index) = a_buf[..shared]
            .iter()
            .zip(&b_buf[..shared])
            .position(|(left_byte, right_byte)| left_byte != right_byte)
        {
            return Ok(FileComparison {
                identical: false,
                first_difference: Some(offset + index as u64),
                left_size,
                right_size,
            });
        }

        if a_read != b_read {
            return Ok(FileComparison {
                identical: false,
                first_difference: Some(offset + shared as u64),
                left_size,
                right_size,
            });
        }

        if a_read == 0 {
            return Ok(FileComparison {
                identical: true,
                first_difference: None,
                left_size,
                right_size,
            });
        }

        offset += a_read as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_first_difference_and_equal_files() {
        let base = std::env::temp_dir();
        let left = base.join(format!("amilcar-compare-a-{}.bin", std::process::id()));
        let right = base.join(format!("amilcar-compare-b-{}.bin", std::process::id()));
        std::fs::write(&left, b"abcdef").unwrap();
        std::fs::write(&right, b"abcxef").unwrap();
        let result = compare_files(&left, &right).unwrap();
        assert!(!result.identical);
        assert_eq!(result.first_difference, Some(3));
        std::fs::write(&right, b"abcdef").unwrap();
        assert!(compare_files(&left, &right).unwrap().identical);
        std::fs::remove_file(left).unwrap();
        std::fs::remove_file(right).unwrap();
    }
}
