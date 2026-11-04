//! AppCraft Error Domain.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppCraftError {
    #[error("Storage failure: {0}")]
    Storage(String),

    #[error("Stigmergic coordination error: {0}")]
    Stigmergy(String),

    #[error("Invalid specification or input: {0}")]
    InvalidInput(String),

    #[error("Compilation or build failure: {0}")]
    BuildFailure(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
