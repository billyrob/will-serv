use std::collections::HashMap;

use crate::http::{BODY_DELIMINATER};

pub fn create_response_string(headers: HashMap<String, String>, return_code: i32, body: Option<& String>) -> String {
    let mut response_string = String::from("HTTP/1.1 ");
    match return_code {
        200 => response_string.push_str("200 OK"),
        400 => response_string.push_str("400 Bad Request"),
        404 => response_string.push_str("404 Not Found"),
        405 => response_string.push_str("405 Method Not Allowed"),
        _ => response_string.push_str("500 Internal Server Error"),
    }
    response_string.push_str("\n");
    for header in headers.keys() {
        match headers.get(header) {
            Some(header_val) => {
                response_string.push_str(header);
                response_string.push_str(": ");
                response_string.push_str(header_val);
                response_string.push_str("\n");
            }
            _ => {}
        }

        
    }
    // HACK: The last newline added in the above loop is not desired in combination with the body deliminater
    // It can be unconditionally removed safely, and it would be a pain to keep a counter in the above loop
    response_string.pop();
    response_string.push_str(BODY_DELIMINATER);
    match body {
        Some(file) => response_string.push_str(file),
        _ => ()

    }
    return response_string;
}

pub fn create_response_string_no_body(headers: HashMap<String, String>, return_code: i32) -> String {
    return create_response_string(headers, return_code, None);
}