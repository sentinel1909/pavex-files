// app/arc/static_files.rs

// dependencies
use mime_guess::from_path;
use pavex::http::HeaderValue;
use pavex::request::RequestHead;
use pavex::response::Response;
use std::borrow::Cow;
use std::env::current_dir;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::{Component::Normal, Path, PathBuf};

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

// helper function to construct a path within the allowed directory
fn safe_join_paths(base_dir: &Path, requested_path: &str) -> Result<PathBuf, Error> {

    let requested_path_buf = Path::new(requested_path).components()
        .fold(PathBuf::new(), |mut path, component| {
            if let Normal(c) = component { path.push(c) };
            path
        });

        let full_path = base_dir.join(requested_path_buf);

        if full_path.exists() {
            let canonical_base = base_dir.canonicalize()?;
            let canonical_path = full_path.canonicalize()?;

            if !canonical_path.starts_with(&canonical_base) {
                return Err(Error::new(
                    ErrorKind::PermissionDenied,
                    "Access to file outsided of allowed directory is not allowed",
                ));
            }
        };

        Ok(full_path)
}

// methods for the StaticAsset type
impl StaticFile {
    pub async fn new(request_head: &RequestHead) -> Result<Self, Error> {
        let base_dir = current_dir()?;
        
        let path = request_head.target.path().trim_start_matches('/');

        if path.contains("..") {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                "Path traversal attempt detected"
            ));
        }

        let file_path = safe_join_paths(&base_dir, path)?;

        let mut file = File::open(&file_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

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
