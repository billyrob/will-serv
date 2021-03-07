use std::net::{TcpStream};
use std::io::{Read, Write};
use std::sync::mpsc::{Receiver};

use crate::http::{HELLO_WORLD_RESPONSE, process};

pub const HTTP_BUFFER_SIZE:usize = 1024 * 1024 * 1;

pub struct WorkerThread {
    pub id: usize,
    pub rx_channel: Receiver<TcpStream>,
    pub input_buffer: Vec<u8>,
    pub output_buffer: Vec<u8>,
}

pub fn run(mut worker: WorkerThread) {
    loop {
        let stream = worker.rx_channel.recv().unwrap();
        handle(&mut worker.input_buffer, stream, worker.id);
    }
}

fn handle(input_buffer: &mut Vec<u8>, mut stream: TcpStream, _: usize) {
    //println!("{}", id);

    let _ = stream.read(input_buffer).expect("Failed to read into buffer");
    //println!("{} bytes", bytes);
    let received = std::str::from_utf8(input_buffer).unwrap();
    process(received);
    stream.write(HELLO_WORLD_RESPONSE).expect("Failed to write response");
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