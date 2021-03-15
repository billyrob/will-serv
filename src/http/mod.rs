use std::collections::HashMap;
use chrono::{DateTime, Utc};
pub const BODY_DELIMINATER: &str = "\r\n\r\n";

#[allow(dead_code)]
#[derive(Debug)]
struct HttpMessage<'a> {
    method: &'a str,
    path: &'a str,
    host: &'a str,
    user_agent: &'a str,
    content_type: &'a str,
    content_length: usize
}

#[allow(unused_variables)]
pub fn process(input_buffer: &str,
    resource_map: & HashMap<String, String>) -> String {
    let mut line_num = 0;
    let mut method: &str = "";
    let mut path: &str = "";
    let mut host: &str = "";
    let mut user_agent: &str = "";
    let mut content_type: &str = "";
    let mut content_length: usize = 0;

    let v: Vec<&str> = input_buffer.splitn(2, BODY_DELIMINATER).collect();
    let headers = v[0];
    let body = v[1];
    for line in headers.lines() {
        if line_num == 0 {
            let pieces: Vec<&str> = line.splitn(3, ' ').collect();
            method = pieces[0];
            path = pieces[1];
        }
        else {
            let pieces: Vec<&str> = line.splitn(2, ':').collect();
            let header_key = pieces[0];
            if header_key == "Host" {
                host = pieces[1].trim();
            }
            if header_key == "User-Agent" {
                user_agent = pieces[1].trim();
            }
            if header_key == "Content-Type" {
                content_type = pieces[1].trim();
            }
            if header_key == "Content-Length" {
                content_length = pieces[1].trim().parse::<usize>().unwrap();
            }
        }
        line_num = line_num + 1;
    }
    let http_message_parsed = HttpMessage{
        method,
        path,
        host,
        user_agent,
        content_type,
        content_length
    };
    let now: DateTime<Utc> = Utc::now();
    if resource_map.contains_key(http_message_parsed.path) {
        let html = resource_map.get(http_message_parsed.path).expect("ASDF");
        return format!(
            "HTTP/1.1 200 OK\nServer: will-serv/0.0.1\nDate: {}\nContent-Type: text/html\nContent-Length: {}{}{}",
            now.to_rfc2822(), html.len(), BODY_DELIMINATER, html)
    }
    else {
        return format!("HTTP/1.1 404 Not Found\nServer: will-serv/0.0.1\nDate: {}{}",
            now.to_rfc2822(), BODY_DELIMINATER);
    }

}