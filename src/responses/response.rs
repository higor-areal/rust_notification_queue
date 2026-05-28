use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseError {
    pub status_code: u16,
    pub message: String,
}

impl ResponseError {
    pub fn bad_request(msg: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::BAD_REQUEST,
            Json(ResponseError {
                status_code: 400,
                message: msg.to_string(),
            }),
        )
    }

    pub fn _not_found(msg: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::NOT_FOUND,
            Json(ResponseError {
                status_code: 404,
                message: msg.to_string(),
            }),
        )
    }

    pub fn internal_server_error(msg: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseError {
                status_code: 500,
                message: msg.to_string(),
            }),
        )
    }
}

#[derive(Serialize)]
pub struct ResponseSuccess<T: Serialize> {
    pub status_code: u16,
    pub data: T,
}

impl<T: Serialize> ResponseSuccess<T> {
    pub fn _ok(data: T) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(ResponseSuccess {
                status_code: 200,
                data,
            }),
        )
    }

    pub fn created(data: T) -> (StatusCode, Json<Self>) {
        (
            StatusCode::CREATED,
            Json(ResponseSuccess {
                status_code: 201,
                data,
            }),
        )
    }
}