// src/routes/assets.rs

// dependencies
use crate::serve_file::ServeFile;
use pavex::http::HeaderValue;
use pavex::response::Response;
use pavex::response::body::{
    TypedBody,
    raw::{Bytes, Full},
};

// implement the TypedBody trait for the StaticAsset type, so that the the response body
// can be created
impl TypedBody for ServeFile {
    type Body = Full<Bytes>;

    fn content_type(&self) -> HeaderValue {
        self.get_serve_file_header_value()
    }

    fn body(self) -> Self::Body {
        Full::new(self.contents.into())
    }
}

// handler function which responds with a 200 OK and the CSS styles
pub fn get(file: ServeFile) -> Response {
    Response::ok().set_typed_body(file)
}
