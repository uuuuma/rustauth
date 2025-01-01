use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("{description}")]
    ValidationError { description: &'static str },

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
