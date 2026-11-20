/// Basic metadata returned by a path inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileInspection {
    pub path: std::path::PathBuf,
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
}
