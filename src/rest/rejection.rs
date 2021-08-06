use crate::errors::ServiceError;
use http::StatusCode;
use serde::Serialize;
use warp::{Rejection, Reply};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not found";
    } else if let Some(e) = err.find::<ServiceError>() {
        log::error!("{}", e);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        log::error!("{}", e);
        code = StatusCode::BAD_REQUEST;
        message = "Bad request";
    } else {
        log::error!("Unknown error");
        log::error!("{:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error";
    }

    let res = ErrorResponse {
        code: code.as_u16(),
        message: message.into(),
    };

    let json_res = warp::reply::json(&res);
    Ok(warp::reply::with_status(json_res, code))
}
