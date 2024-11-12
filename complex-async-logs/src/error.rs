use thiserror::Error;

use crate::app;

#[derive(Debug, Error)]
pub enum Error {
    #[error("CrosstermEvent error")]
    CrosstermEvent,
    #[error("JoinError")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<app::Event>),
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("Unexpected: {0}")]
    Unexpected(String),
}
