use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Database-Error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Validation-Error: {0}")]
    Validation(String),
}

pub type CoreResult<T> = Result<T, CoreError>;