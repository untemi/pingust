use std::io;

pub type Result<T> = std::result::Result<T, Error>;
pub type AnyError = anyhow::Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request timeout")]
    Timeout,

    #[error("{0}")]
    Disconnect(#[from] io::Error),

    #[error("{0}")]
    Others(#[from] AnyError),
}
