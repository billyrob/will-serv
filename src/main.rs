use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::collections::HashMap;
use std::{fs, thread};
use std::path::Path;
// TODO: Remove non-std import!
use rand::Rng;
mod worker;
mod http;


fn load_web_resources<P>(resource_map: &mut HashMap<String, String>, _: P)
where P:AsRef<Path> {
    /*
    for entry in fs::read_dir(fs_path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            load_web_resources(resource_map, path);
        }
        else {
            if path.ends_with(".html") {
                resource_map.insert(k: K, v: V)
            }
        }
    }
    */
    let a = fs::read_to_string("web/index.html").expect("Unable to read file");
    let b = fs::read_to_string("web/test/index.html").expect("Unable to read file");
    resource_map.insert("/".to_string(), a);
    resource_map.insert("/test".to_string(), b);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut web_resources: HashMap<String, String> = HashMap::new();

    load_web_resources(&mut web_resources, "web");

    const NUM_WORKERS:usize = 20;

    let mut tx_channels: Vec<Sender<TcpStream>> = Vec::with_capacity(NUM_WORKERS);
    for n in 0..NUM_WORKERS {
        let (tx, rx): (Sender<TcpStream>, Receiver<TcpStream>) = mpsc::channel();
        tx_channels.push(tx);
        let mut w = worker::WorkerThread{
            id: n,
            rx_channel: rx,
            input_buffer: vec![0u8; worker::HTTP_BUFFER_SIZE],
            resource_map: web_resources.clone(),
        };
        thread::spawn(move || worker::run(&mut w));
    }
    
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match stream.try_clone() {
                    Ok(clone) => {
                        let worker_index:usize = rng.gen_range(0..NUM_WORKERS);
                        tx_channels[worker_index].send(clone).unwrap();
                    },
                    Err(e) => {
                        println!("Problem Cloning the stream: {:?}", e);
                    }
                };
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
