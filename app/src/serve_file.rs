// app/arc/static_files.rs

// dependencies
use crate::configuration::AppConfig;
use mime_guess::from_path;
use pavex::http::HeaderValue;
use pavex::request::RequestHead;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use tokio::fs::read;
use tokio::io::Error;

// struct type to represent a static asset, CCSS, JS, an image, or anything else
#[derive(Debug, Clone)]
pub struct ServeFile {
    pub contents: Vec<u8>,
    pub mime_type: Cow<'static, str>,
}

// methods for the ServeFile type
impl ServeFile {
    pub async fn new(config: &AppConfig, request_head: &RequestHead) -> Result<Self, Error> {
        let serve_file_subdir = Path::new(config.static_files.dir.as_ref());

        let request_target = request_head.target.path().trim_start_matches('/');

        let filename = Path::new(request_target)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut file_path = PathBuf::from(serve_file_subdir);
        file_path.push(filename);

        let contents: Vec<u8> = read(&file_path).await?;

        let mime_type = from_path(&file_path)
            .first_or_octet_stream()
            .to_string()
            .into();

        let serve_file = Self {
            contents,
            mime_type,
        };

        Ok(serve_file)
    }

    pub fn get_serve_file_header_value(&self) -> HeaderValue {
        HeaderValue::from_str(&self.mime_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"))
    }
}
