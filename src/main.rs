use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
// TODO: Remove non-std import!
use rand::Rng;
mod worker;
mod internal_http;

fn main() {
    let mut rng = rand::thread_rng();
    const NUM_WORKERS:usize = 20;

    let mut tx_channels: Vec<Sender<TcpStream>> = Vec::with_capacity(NUM_WORKERS);
    for n in 0..NUM_WORKERS {
        let (tx, rx): (Sender<TcpStream>, Receiver<TcpStream>) = mpsc::channel();
        tx_channels.push(tx);
        let w = worker::WorkerThread{
            id: n,
            rx_channel: rx,
            input_buffer: vec![0u8; worker::HTTP_BUFFER_SIZE],
            output_buffer: vec![0u8; worker::HTTP_BUFFER_SIZE],
        };
        thread::spawn(move || worker::run(w));
    }
    
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_clone = match stream.try_clone() {
                    Ok(clone) => clone,
                    Err(e) => panic!("Problem Cloning the stream: {:?}", e),
                };
                let worker_index:usize = rng.gen_range(0..NUM_WORKERS);
                tx_channels[worker_index].send(stream_clone).unwrap();
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
