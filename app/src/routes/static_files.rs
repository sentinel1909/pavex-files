// src/routes/css_asset.rs

// dependencies
use crate::static_file::StaticFile;
use pavex::http::HeaderValue;
use pavex::response::Response;
use pavex::response::body::{
    TypedBody,
    raw::{Bytes, Full},
};

// implement the TypedBody trait for the StaticAsset type, so that the the response body
// can be created
impl TypedBody for StaticFile {
    type Body = Full<Bytes>;

    fn content_type(&self) -> HeaderValue {
        self.get_asset_header_value()
    }

    fn body(self) -> Self::Body {
        Full::new(self.asset_data.into())
    }
}

// handler function which responds with a 200 OK and the CSS styles
pub fn get(file: StaticFile) -> Response {
    Response::ok().set_typed_body(file)
}
