//! Filesystem primitives for safe, local file operations.
pub mod compare;
pub mod directory;
pub mod duplicates;
pub mod hash;
pub mod metadata;
pub mod operations;
pub mod reader;
pub mod writer;

pub use compare::{compare_files, FileComparison};
pub use directory::{find_files, DirectoryOptions, FileFilter};
pub use duplicates::{find_duplicates, DuplicateGroup};
pub use hash::sha256_file;
pub use metadata::{inspect, FileMetadata};
pub use operations::{copy_file, create_directory, move_path, remove_empty_directory, remove_file, rename_path};
pub use reader::{copy_file_contents, open_buffered, read_to_string_limited};
pub use writer::{write_bytes, WriteMode};
