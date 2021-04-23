use std::collections::HashMap;
use chrono::{DateTime, Utc};

// will-serv only serves files currently. No POSTs, PUTs, DELETEs, etc
const ALLOWED_METHODS: &'static [&'static str] = &["GET", "HEAD"];
const BODY_DELIMINATER: &str = "\r\n\r\n";
const VERSION_STR: &str = env!("CARGO_PKG_VERSION");

pub fn process_request(input_buffer: &str, resource_map: & HashMap<String, String>) -> String {
    // Process input_buffer into HTTP concepts
    let mut header_map: HashMap<&str, &str> = HashMap::new();
    let mut line_num = 0;
    let mut method: &str = "";
    let mut path: &str = "";

    let v: Vec<&str> = input_buffer.splitn(2, BODY_DELIMINATER).collect();
    // The request must be delimited by the BODY_DELIMINATER
    if v.len() < 2 {
        let now: DateTime<Utc> = Utc::now();
        return format!("HTTP/1.1 400 Bad Request\nServer: will-serv/{}\nDate: {}{}",
            VERSION_STR, now.to_rfc2822(), BODY_DELIMINATER);
    }
    let headers = v[0];
    let _body = v[1];
    for line in headers.lines() {
        if line_num == 0 {
            // Read the Request Line
            // https://tools.ietf.org/html/rfc7230#section-3.1.1
            let pieces: Vec<&str> = line.splitn(3, ' ').collect();
            method = pieces[0];
            path = pieces[1];
        }
        else {
            // Read Headers
            // https://tools.ietf.org/html/rfc7230#section-3.2
            let pieces: Vec<&str> = line.splitn(2, ':').collect();
            let header_key = pieces[0];
            let header_value = pieces[1].trim();
            header_map.insert(header_key, header_value);
        }
        line_num = line_num + 1;
    }

    // Act on parsed elements
    let now: DateTime<Utc> = Utc::now();
    let mut file: Option<&String> = None;

    if !ALLOWED_METHODS.contains(&method) {
        let allow_header = ALLOWED_METHODS.join(", ");
        return format!(
            "HTTP/1.1 405 Method Not Allowed\nAllow: {}\nServer: will-serv/{}\nDate: {}{}",
            allow_header, VERSION_STR, now.to_rfc2822(), BODY_DELIMINATER);
    }

    if resource_map.contains_key(path) {
        file = Some(resource_map.get(path).expect("Failed to request HTML"));
    }
    else {
        if path.ends_with("/") {
            let mut requested_file = String::from(path);
            requested_file.push_str("index.html");
            if resource_map.contains_key(&requested_file) {
                file = Some(resource_map.get(&requested_file).expect("Failed to request HTML"));
            }
        }
    }

    match file {
        None => return format!(
            "HTTP/1.1 404 Not Found\nServer: will-serv/{}\nDate: {}{}",
            VERSION_STR, now.to_rfc2822(), BODY_DELIMINATER),
        Some(f) => return format!(
            "HTTP/1.1 200 OK\nServer: will-serv/{}\nDate: {}\nContent-Type: text/html\nContent-Length: {}{}{}",
            VERSION_STR, now.to_rfc2822(), f.len(), BODY_DELIMINATER, f),
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
        assert!(response_html.contains("Allow: GET, HEAD"));
    }
}