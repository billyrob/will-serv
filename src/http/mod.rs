use std::collections::HashMap;
use chrono::{DateTime, Utc};
pub const BODY_DELIMINATER: &str = "\r\n\r\n";

pub fn process_request(input_buffer: &str, resource_map: & HashMap<String, String>) -> String {
    let mut header_map: HashMap<&str, &str> = HashMap::new();
    let mut line_num = 0;
    let mut _method: &str = "";
    let mut path: &str = "";

    let v: Vec<&str> = input_buffer.splitn(2, BODY_DELIMINATER).collect();
    let headers = v[0];
    let _body = v[1];
    for line in headers.lines() {
        if line_num == 0 {
            let pieces: Vec<&str> = line.splitn(3, ' ').collect();
            _method = pieces[0];
            path = pieces[1];
        }
        else {
            let pieces: Vec<&str> = line.splitn(2, ':').collect();
            let header_key = pieces[0];
            let header_value = pieces[1].trim();
            header_map.insert(header_key, header_value);
        }
        line_num = line_num + 1;
    }
    let now: DateTime<Utc> = Utc::now();
    if resource_map.contains_key(path) {
        let html = resource_map.get(path).expect("ASDF");
        return format!(
            "HTTP/1.1 200 OK\nServer: will-serv/0.0.1\nDate: {}\nContent-Type: text/html\nContent-Length: {}{}{}",
            now.to_rfc2822(), html.len(), BODY_DELIMINATER, html)
    }
    else {
        return format!("HTTP/1.1 404 Not Found\nServer: will-serv/0.0.1\nDate: {}{}",
            now.to_rfc2822(), BODY_DELIMINATER);
    }

}