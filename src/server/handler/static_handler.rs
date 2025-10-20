use crate::core::Response;
use std::fs;
use std::path::{Path, PathBuf};

pub fn serve_file(root: &Path, request_path: &str) -> Response {
    let mut full_path = PathBuf::from(root);
    let request_path = request_path.trim_start_matches('/');
    full_path.push(request_path);

    if full_path.is_dir() {
        full_path.push("index.html");
    }

    match fs::read(&full_path) {
        Ok(contents) => {
            let content_type = mime_guess::from_path(&full_path).first_or_text_plain();

            Response::new(200, "OK")
                .header("Content-Type", content_type.as_ref())
                .with_body(contents)
        }
        Err(_) => Response::new(404, "Not Found")
            .header("Content-Type", "text/plain")
            .with_body(b"404 Not Found".to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_serve_file() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create a test file
        let test_file = root.join("index.html");
        fs::write(&test_file, "<h1>Hello, World!</h1>").unwrap();

        let response = serve_file(root, "/index.html");
        assert_eq!(response.status_code, 200);
        assert_eq!(response.headers.get("Content-Type"), Some(&"text/html".to_string()));
        assert_eq!(response.body, b"<h1>Hello, World!</h1>");
    }

    #[test]
    fn test_serve_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        let response = serve_file(root, "/non_existent.html");
        assert_eq!(response.status_code, 404);
        assert_eq!(response.body, b"404 Not Found");
    }
}
