pub type Result<T> = std::result::Result<T, Error>;
pub type AnyError = anyhow::Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request timeout")]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error("{0}")]
    Disconnect(#[from] tokio::io::Error),

    #[error("{0}")]
    Others(#[from] AnyError),
}
