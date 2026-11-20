//! Filesystem primitives for safe, local file operations.
pub mod compare;
pub mod directory;
pub mod metadata;
pub mod reader;
pub mod writer;

pub use compare::{compare_files, FileComparison};
pub use directory::{find_files, DirectoryOptions, FileFilter};
pub use metadata::{inspect, FileMetadata};
pub use reader::{copy_file_contents, open_buffered, read_to_string_limited};
pub use writer::{write_bytes, WriteMode};
