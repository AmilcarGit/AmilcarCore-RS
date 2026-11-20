/// Runtime limits for an engine instance.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Maximum file size accepted by inspection operations.
    pub max_file_size: u64,
}
impl Default for EngineConfig {
    fn default() -> Self { Self { max_file_size: 64 * 1024 * 1024 } }
}
