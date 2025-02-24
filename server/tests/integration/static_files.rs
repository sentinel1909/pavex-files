// tests/integration/static_asset.rs

use crate::helpers::TestApi;
use pavex::http::StatusCode;

#[tokio::test]
async fn static_files_works() {
    let api = TestApi::spawn().await;

    let test_cases = [("screen.css", "text/css"), ("script.js", "text/javascript")];

    for (filename, expected_content_type) in test_cases.iter() {
        let response = api.get_static_asset(filename).await;

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Unexpected status code for {}",
            filename
        );

        let response_header = response
            .headers()
            .get("Content-Type")
            .unwrap_or_else(|| panic!("Expected Content-Type header in response for {}", filename));

        let response_header_str = response_header
            .to_str()
            .expect("Unable to convert Content-Type header to string");

        assert_eq!(
            response_header_str, *expected_content_type,
            "Unexpected Content-Type for {}: expected {}, got {}",
            filename, expected_content_type, response_header_str
        );
    }
}
