// app/arc/static_files.rs

// dependencies
use mime_guess::from_path;
use pavex::http::HeaderValue;
use pavex::request::RequestHead;
use pavex::response::Response;
use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// struct type to represent a static asset, CCSS, JS, an image, or anything else
#[derive(Debug, Clone)]
pub struct StaticFile {
    pub asset_data: Vec<u8>,
    pub asset_mime_type: Cow<'static, str>,
}

// error handler for the StaticFile::new() method
pub async fn io_error2response(e: &pavex::Error) -> Response {
    Response::internal_server_error().set_typed_body(e.to_string())
}

// methods for the StaticAsset type
impl StaticFile {
    pub async fn new(request_head: &RequestHead) -> Result<Self, std::io::Error> {
        let request_path = Path::new(request_head.target.path().trim_start_matches("/"));
        let base_dir = std::env::current_dir()?;
        let full_path = Path::new(&base_dir).join(request_path);
        let mut file = File::open(&full_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let mime_type = from_path(&full_path)
            .first_or_octet_stream()
            .to_string()
            .into();

        let static_asset = Self {
            asset_data: contents,
            asset_mime_type: mime_type,
        };

        Ok(static_asset)
    }

    pub fn get_header_value(&self) -> HeaderValue {
        HeaderValue::from_str(&self.asset_mime_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"))
    }
}
