use std::fmt;

#[derive(Debug)]
pub enum Error {
    DbError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DbError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
