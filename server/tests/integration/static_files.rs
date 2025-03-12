// tests/integration/static_asset.rs

use crate::helpers::TestApi;
use pavex::http::StatusCode;

#[tokio::test]
async fn static_files_works() {
    let api = TestApi::spawn().await;

    let test_cases = [
        ("index.html", "text/html"),
        ("screen.css", "text/css"),
        ("script.js", "text/javascript"),
    ];

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

#[tokio::test]
async fn non_existent_files_return_404() {
    let api = TestApi::spawn().await;

    let filename = "shouldntexist.txt";
    let response = api.get_static_asset(filename).await;

    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "Expected 404 status code for non-existent file {}",
        filename
    );
}

#[tokio::test]
async fn path_traversal_attempts_are_prevented() {
    let api = TestApi::spawn().await;

    // Attempt to access a file outside of the assets directory
    let filename = "../secrets.txt";
    let response = api.get_static_asset(filename).await;

    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "Expected 404 status code for path traversal attempt with {}",
        filename
    );

    // Another test case to check different levels of path traversal
    let filename2 = "../../secrets.txt";
    let response2 = api.get_static_asset(filename2).await;

    assert_eq!(
        response2.status(),
        StatusCode::NOT_FOUND,
        "Expected 404 status code for another path traversal attempt with {}",
        filename2
    );
}
