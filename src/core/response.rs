use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: u16, reason_phrase: &str) -> Self {
        Self {
            status_code,
            reason_phrase: reason_phrase.to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        let body_bytes = body.into();
        self.headers.insert("Content-Length".to_string(), body_bytes.len().to_string());
        self.body = body_bytes;
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Serialize the full HTTP response into bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = String::new();

        // Status line
        let _ = write!(
            response,
            "HTTP/1.1 {} {}\r\n",
            self.status_code, self.reason_phrase
        );

        // Headers
        for (key, value) in &self.headers {
            let _ = write!(response, "{}: {}\r\n", key, value);
        }

        // End of headers
        response.push_str("\r\n");

        let mut bytes = response.into_bytes();
        bytes.extend_from_slice(&self.body);
        bytes
    }
    
    pub fn redirect(location: String, status_code: u16) -> Self {
        let reason_phrase = match status_code {
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            _ => "Redirect",
        }
        .to_string();

        let mut headers = HashMap::new();
        headers.insert("Location".to_string(), location.clone());
        headers.insert("Content-Type".to_string(), "text/html".to_string());

        let body = format!(
            "<html><body><h1>{} Redirect</h1><p>Redirecting to <a href=\"{}\">{}</a></p></body></html>",
            status_code, location, location
        );

        Self {
            status_code,
            reason_phrase,
            headers,
            body: body.into_bytes(),
        }
    }

    // Generate html error response
    pub fn error(status_code: u16, reason_message: String) -> Self {
        let mut defaulting_reason = false;
        let reason = if reason_message.trim().is_empty() {
            defaulting_reason = true;
            default_reason_phrase(status_code).to_string()
        } else {
            reason_message
        };
    
        let title = format!("{} {}", status_code, reason);
        let paragraph = if defaulting_reason {
            format!("The server returned status {}.", status_code)
        } else {
            reason.clone()
        };
        let body = format!(
            "<!DOCTYPE html>\n<html>\n<head><meta charset=\"utf-8\"><title>{title}</title></head>\n<body>\n  <h1>{title}</h1>\n  <p>{paragraph}</p>\n</body>\n</html>\n",
            title = title,
            paragraph = paragraph,
        );
    
        Response::new(status_code, &reason)
            .header("Content-Type", "text/html; charset=utf-8")
            .with_body(body)
    }
}

fn default_reason_phrase(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        418 => "I'm a teapot",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Error",
    }
}
