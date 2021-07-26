use std::net::{TcpStream};
use std::io::{Read, Write};
use std::sync::mpsc::{Receiver};
use std::collections::HashMap;
use log;

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
        let stream = worker.rx_channel.recv();
        match stream {
            Ok(stream) => {
                handle_tcp_stream(worker, stream);
            }
            Err(e) => {
                log::error!("Failed to send to worker thread {:?}", e);
            }
        }
        
    }
}

fn handle_tcp_stream(worker: &mut WorkerThread, mut stream: TcpStream) {
    let peer_addr = stream.peer_addr();
    if peer_addr.is_ok() {
        log::info!("Received request from: {}", peer_addr.unwrap());
    }    
    let res = stream.read(&mut worker.input_buffer);
    let received: Option<&str>;
    match res {
        Ok(_) => {
            match std::str::from_utf8(&mut worker.input_buffer) {
                Ok(x) => received = Some(x),
                Err(e) => {
                    log::error!("Input data not valid utf8s {:?}", e);
                    return;
                }
            }
        },
        Err(e) => {
            log::error!("Failed to read from stream {:?}", e);
            return;
        }
    }
    match received {
        None => {
            log::error!("Was unable to process the request");
            match stream.shutdown(std::net::Shutdown::Both) {
                Ok(x) => x,
                Err(e) => {
                    log::error!("Failed to shutdown connection {:?}", e);
                }
            };
        },
        Some(r) => {
            let response = process_request(r, &worker.resource_map);
            let _ = stream.write(response.as_bytes());
            match stream.flush() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("Failed to flush data {:?}", e);
                }
            };
            match stream.shutdown(std::net::Shutdown::Both) {
                Ok(x) => x,
                Err(e) => {
                    log::error!("Failed to shutdown connection {:?}", e);
                }
            };
        }
    }
    
}