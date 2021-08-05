use std::error::Error;
use std::fmt;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    PgDb(tokio_postgres::error::Error),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PgDb(err) => write!(f, "[ServiceError::PgDb] {:}", err),
        }
    }
}

impl Error for ServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::PgDb(err) => Some(err),
        }
    }
}

impl From<tokio_postgres::error::Error> for ServiceError {
    fn from(err: tokio_postgres::error::Error) -> Self {
        Self::PgDb(err)
    }
}
