use crate::errors::ServiceError;
use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};
use std::error::Error;
use std::fmt;

pub type AppGraphQLResult<T> = Result<T, AppGraphQLError>;

#[derive(Debug)]
pub enum AppGraphQLError {
    Service(ServiceError),
}

impl<S: ScalarValue> IntoFieldError<S> for AppGraphQLError {
    fn into_field_error(self) -> FieldError<S> {
        log::error!("{}", self);
        FieldError::new("Server error", graphql_value!({"code": "500"}))
    }
}

impl fmt::Display for AppGraphQLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Service(err) => write!(f, "[AppGraphQLError::ServiceError] {:}", err),
        }
    }
}

impl Error for AppGraphQLError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Service(err) => Some(err),
        }
    }
}

impl From<ServiceError> for AppGraphQLError {
    fn from(err: ServiceError) -> Self {
        Self::Service(err)
    }
}
