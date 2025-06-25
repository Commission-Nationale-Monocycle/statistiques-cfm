use thiserror::Error;

pub type Result<T, E = ApplicationError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Sheet(#[from] calamine::XlsError),
    #[error(transparent)]
    Deserialization(#[from] calamine::DeError),
    #[error("The file has no header row.")]
    NoHeaders,
    #[error("The row is misformatted.")]
    MisformattedRow,
    #[error("A cell has a wrong format: {0}")]
    WrongFormat(String),
}
