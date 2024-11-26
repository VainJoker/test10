//! Error types for the crate
use thiserror::Error;

/// The error type for this crate
#[derive(Error, Debug)]
pub enum Error {}

/// Convenience type alias for this crate's error type
pub type Result<T> = std::result::Result<T, Error>;
