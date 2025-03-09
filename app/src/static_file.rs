// app/arc/static_files.rs

// dependencies
use crate::configuration::AppConfig;
use mime_guess::from_path;
use pavex::http::HeaderValue;
use pavex::request::RequestHead;
use pavex::response::Response;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use tokio::fs::read;
use tokio::io::Error;

// struct type to represent a static asset, CCSS, JS, an image, or anything else
#[derive(Debug, Clone)]
pub struct StaticFile {
    pub asset_data: Vec<u8>,
    pub asset_mime_type: Cow<'static, str>,
}

// error handler for the StaticFile::new() method
pub async fn io_error2response(e: &pavex::Error) -> Response {
    Response::not_found().set_typed_body(e.to_string())
}

// methods for the StaticAsset type
impl StaticFile {
    pub async fn new(config: &AppConfig, request_head: &RequestHead) -> Result<Self, Error> {
        let assets_subdir = Path::new(config.static_files.dir.as_ref());

        let request_target = request_head.target.path().trim_start_matches('/');

        let filename = Path::new(request_target)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut file_path = PathBuf::from(assets_subdir);
        file_path.push(filename);

        let contents: Vec<u8> = read(&file_path).await?;

        let mime_type = from_path(&file_path)
            .first_or_octet_stream()
            .to_string()
            .into();

        let static_asset = Self {
            asset_data: contents,
            asset_mime_type: mime_type,
        };

        Ok(static_asset)
    }

    pub fn get_asset_header_value(&self) -> HeaderValue {
        HeaderValue::from_str(&self.asset_mime_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"))
    }
}
