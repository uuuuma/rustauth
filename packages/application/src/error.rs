use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("{description}")]
    InvalidParameters { description: &'static str },

    #[error("user already exist")]
    UserAlreadyExistError,

    #[error("user is not found")]
    UserIsNotFoundError,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
