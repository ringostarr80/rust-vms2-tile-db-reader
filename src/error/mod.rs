use std::fmt;

#[derive(Debug)]
pub enum Error {
    DbError {
        message: String,
        source: Option<Box<dyn std::error::Error + 'static>>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DbError { message, .. } => write!(f, "Database error: {}", message),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DbError {
                source: Some(err), ..
            } => Some(&**err),
            _ => None,
        }
    }
}
