#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod error;
pub mod ids;
pub mod time;

pub use error::DomainError;
