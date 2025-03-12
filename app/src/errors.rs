// src/errors.rs

// dependencies
use pavex::Error;
use pavex::response::Response;

// universal error handler for an IO error
pub async fn io_error2response(e: &Error) -> Response {
    Response::not_found().set_typed_body(e.to_string())
}
