//! Public API and orchestration layer.
mod config;
mod engine;
mod error;
mod result;
pub use config::EngineConfig;
pub use engine::Engine;
pub use error::{AmilcarError, Result};
pub use result::FileInspection;
