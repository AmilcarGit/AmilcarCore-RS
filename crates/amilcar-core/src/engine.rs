use std::path::Path;

use crate::{AmilcarError, EngineConfig, FileInspection, Result};

/// Main entry point for local processing operations.
#[derive(Debug, Clone, Default)]
pub struct Engine {
    config: EngineConfig,
}

impl Engine {
    /// Create an engine with default limits.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an engine with explicit configuration.
    pub fn with_config(config: EngineConfig) -> Self {
        Self { config }
    }

    /// Return the active configuration.
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Inspect filesystem metadata without reading file contents.
    pub fn inspect_path(&self, path: impl AsRef<Path>) -> Result<FileInspection> {
        let metadata = amilcar_fs::metadata::inspect(path.as_ref())?;
        if metadata.is_file && metadata.size > self.config.max_file_size {
            return Err(AmilcarError::FileTooLarge {
                size: metadata.size,
                limit: self.config.max_file_size,
            });
        }

        Ok(FileInspection {
            path: metadata.path,
            size: metadata.size,
            is_file: metadata.is_file,
            is_directory: metadata.is_directory,
        })
    }
}
