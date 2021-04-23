use std::collections::HashMap;

use crate::http::{BODY_DELIMINATER};

pub struct HttpRequest {
    pub headers: HashMap<String, String>,
    pub method: String,
    pub path: String,
    pub body: String,
}

pub fn process_buffer(input_buffer: &str) -> Result<HttpRequest, ()> {
    let mut header_map: HashMap<String, String> = HashMap::new();
    let mut line_num = 0;
    let mut method: &str = "";
    let mut path: &str = "";

    let v: Vec<&str> = input_buffer.splitn(2, BODY_DELIMINATER).collect();
    // The request must be delimited by the BODY_DELIMINATER
    if v.len() < 2 {
        return Err(());
    }
    let headers = v[0];
    let body = v[1];
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
            header_map.insert(String::from(header_key), String::from(header_value));
        }
        line_num = line_num + 1;
    }
    Ok(HttpRequest {
        headers: header_map,
        method: String::from(method),
        path: String::from(path),
        body: String::from(body),
    })
}




    