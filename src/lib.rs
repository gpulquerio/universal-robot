//! Universal Robots API
//!

#[cfg(test)]
mod test;

mod dashboard;
mod physical;
mod rolling_buffer;
mod rtde;

pub use rtde::data;
pub use rtde::types;
use rtde::types::PackageType;
pub use rtde::Rtde;

pub mod prelude {
    pub(crate) use crate::Error;
    pub(crate) use std::thread::sleep;
    pub(crate) use std::time::{Duration, Instant};
    pub(crate) type Result<T> = core::result::Result<T, Error>;

    pub use crate::dashboard::Dashboard;
    pub use crate::physical::UniversalRobot;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Static(&'static str),
    #[error("Unexpected Response: '{0}'")]
    UnexpectedResponse(String),
    #[error("Connection closed unexpectedly")]
    ConnectionLost,
    #[error("Exceeded maximum read attempts for package, expected {0:?}")]
    MaxReads(PackageType),
    #[error("timeout powering on. {0:?} after {1:?}")]
    Timeout(String, u64),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
