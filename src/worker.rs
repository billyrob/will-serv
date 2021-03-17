use std::net::{TcpStream};
use std::io::{Read, Write};
use std::sync::mpsc::{Receiver};
use std::collections::HashMap;

use crate::http::{process_request};

// Max message size in bytes
// 1 MB for the body, 1 MB for the headers 
pub const HTTP_BUFFER_SIZE:usize = 1024 * 1024 * 2;

pub struct WorkerThread {
    pub id: usize,
    pub rx_channel: Receiver<TcpStream>,
    pub input_buffer: Vec<u8>,
    pub resource_map: HashMap<String, String>,
}

pub fn run(worker: &mut WorkerThread) {
    loop {
        let stream = worker.rx_channel.recv().unwrap();
        handle_tcp_stream(worker, stream);
    }
}

fn handle_tcp_stream(worker: &mut WorkerThread, mut stream: TcpStream) {

    stream.read(&mut worker.input_buffer).expect("Failed to read into buffer");
    let received = std::str::from_utf8(&mut worker.input_buffer).unwrap();
    let response = process_request(received, &worker.resource_map);
    stream.write(response.as_bytes()).expect("Failed to write response");
    match stream.flush() {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to flush data {:?}", e);
        }
    };
    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to shutdown connection {:?}", e);
        }
    };
}