use std::collections::HashMap;
use chrono::{DateTime, Utc};
use log;

pub mod request;
pub mod response;

// will-serv only serves files currently. No POSTs, PUTs, DELETEs, etc
const ALLOWED_METHODS: &'static [&'static str] = &["GET"];
const BODY_DELIMINATER: &str = "\r\n\r\n";
const VERSION_STR: &str = env!("CARGO_PKG_VERSION");

#[allow(dead_code)]
pub fn create_short_circuit_error_response() -> String {
    let now: DateTime<Utc> = Utc::now();
    let mut response_headers: HashMap<String, String> = HashMap::new();
    response_headers.insert(String::from("Server"), format!("will-serv/{}", VERSION_STR));
    response_headers.insert(String::from("Date"), now.to_rfc2822());
    return response::create_response_string_no_body(response_headers, 400);
}

pub fn process_request(input_buffer: &str, resource_map: & HashMap<String, String>) -> String {
    // Process input_buffer into struct
    let req: request::HttpRequest;
    let now: DateTime<Utc> = Utc::now();
    let mut response_headers: HashMap<String, String> = HashMap::new();
    response_headers.insert(String::from("Server"), format!("will-serv/{}", VERSION_STR));
    response_headers.insert(String::from("Date"), now.to_rfc2822());
    match request::process_buffer(input_buffer) {
        Err(_) => {
            log::info!("Return Code: 400");
            return response::create_response_string_no_body(response_headers, 400);
        },
        Ok(r) => req = r,
    }
    let mut file: Option<&String> = None;

    if !ALLOWED_METHODS.contains(&req.method.as_str()) {
        let allow_header = ALLOWED_METHODS.join(", ");
        response_headers.insert(String::from("Allow"), allow_header);
        log::info!("Return Code: 405");
        return response::create_response_string_no_body(response_headers, 405);
    }
    if resource_map.contains_key(&req.path) {
        file = Some(resource_map.get(&req.path).unwrap());
    }
    else {
        if req.path.ends_with("/") {
            let mut requested_file = String::from(req.path);
            requested_file.push_str("index.html");
            if resource_map.contains_key(&requested_file) {
                file = Some(resource_map.get(&requested_file).unwrap());
            }
        }
    }
    match file {
        None => {
            log::info!("Return Code: 404");
            return response::create_response_string_no_body(response_headers, 404);
        },
        Some(f) => {
            response_headers.insert(String::from("Content-Type"), String::from("text/html; charset=UTF-8"));
            response_headers.insert(String::from("Content-Length"), f.len().to_string());
            log::info!("Return Code: 200");
            return response::create_response_string(response_headers, 200, Some(f));
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_happy_path() {
        let mut web_resources: HashMap<String, String> = HashMap::new();
        web_resources.insert("/".to_string(), "<h1> TEST </h1>".to_string());
        let get_request_buffer: &str = "GET / 1.1\n\r\n\r\n";
        let response_html = process_request(get_request_buffer, & web_resources);

        assert!(response_html.contains("HTTP/1.1 200 OK"));
        assert!(response_html.contains("<h1> TEST </h1>"));
    }

    #[test]
    fn test_404() {
        let mut web_resources: HashMap<String, String> = HashMap::new();
        web_resources.insert("/".to_string(), "<h1> TEST </h1>".to_string());
        let get_request_buffer: &str = "GET /NONEXISTENT 1.1\n\r\n\r\n";
        let response_html = process_request(get_request_buffer, & web_resources);

        assert!(response_html.contains("HTTP/1.1 404 Not Found"));
    }

    #[test]
    fn test_many_headers() {
        let mut web_resources: HashMap<String, String> = HashMap::new();
        web_resources.insert("/".to_string(), "<h1> TEST </h1>".to_string());
        let get_request_buffer: &str = "GET / 1.1
        QWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\nQWER: ASDF\r\n\r\n";
        let response_html = process_request(get_request_buffer, & web_resources);

        assert!(response_html.contains("HTTP/1.1 200 OK"));
        assert!(response_html.contains("<h1> TEST </h1>"));
    }

    #[test]
    fn test_malformed() {
        let mut web_resources: HashMap<String, String> = HashMap::new();
        web_resources.insert("/".to_string(), "<h1> TEST </h1>".to_string());
        let malformed_request: &str = "INVALIDMETHOD/NONEXISTENT1.1\n\r\r\n";
        let response_html = process_request(malformed_request, & web_resources);

        assert!(response_html.contains("HTTP/1.1 400 Bad Request"));
    }

    #[test]
    fn test_method_parsing() {
        let mut web_resources: HashMap<String, String> = HashMap::new();
        web_resources.insert("/".to_string(), "<h1> TEST </h1>".to_string());
        let get_request_buffer: &str = "GET / 1.1\n\r\n\r\n";
        let response_html = process_request(get_request_buffer, & web_resources);

        assert!(response_html.contains("HTTP/1.1 200 OK"));
        assert!(response_html.contains("<h1> TEST </h1>"));

        let post_request_buffer: &str = "POST / 1.1\n\r\n\r\n";
        let response_html = process_request(post_request_buffer, & web_resources);

        assert!(response_html.contains("HTTP/1.1 405 Method Not Allowed"));
        assert!(response_html.contains("Allow: GET"));
    }
}