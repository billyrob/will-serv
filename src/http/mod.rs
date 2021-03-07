
pub const HELLO_WORLD_RESPONSE: &[u8; 121] = 
b"HTTP/1.1 200 OK
Server: will-serv/0.0.1
Date: Mon, 15 Feb 2021
Content-Type: text/html
Content-Length: 14

Hello, World!
";

#[allow(dead_code)]
#[derive(Debug)]
struct HttpMessage<'a> {
    method: &'a str,
    uri: &'a str,
    host: &'a str,
    user_agent: &'a str,
    content_type: &'a str,
    content_length: usize
}

#[allow(unused_variables)]
pub fn process(input_buffer: &str) {
    let mut line_num = 0;
    let mut in_headers = true;
    let mut method: &str = "";
    let mut uri: &str = "";
    let mut host: &str = "";
    let mut user_agent: &str = "";
    let mut content_type: & str = "";
    let mut content_length: usize = 0;
    for line in input_buffer.lines() {
        if line == "" {
            in_headers = false;
        }
        if in_headers {
            if line_num == 0 {
                let pieces: Vec<&str> = line.splitn(3, ' ').collect();
                method = pieces[0];
                uri = pieces[1];
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
        }
        else {
            //println!("{}", line);
        }
        line_num = line_num + 1;
    }
    let http_message_parsed = HttpMessage{
        method,
        uri,
        host,
        user_agent,
        content_type,
        content_length
    };
    //println!("{:?}", http_message_parsed);
}