use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::collections::HashMap;
use std::{fs, thread};
use std::path::Path;
use rand::Rng;
mod worker;
mod http;


fn load_web_resources<P>(resource_map: &mut HashMap<String, String>, fs_path: P, original_root: & String)
where P:AsRef<Path> {
    for entry in fs::read_dir(fs_path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            load_web_resources(resource_map, path, & original_root);
        }
        else {
            // Go from a path of 'web/test/index.html` to `/test/index.html` being the key
            // It is up to the request handler to take a user request for `/test`
            // and translate that to `/test/index.html`
            let key = path.strip_prefix(original_root).unwrap();
            let mut key_string = String::from("/");
            key_string.push_str(key.to_str().unwrap());
            let f = fs::read_to_string(path).expect("Unable to read file");
            resource_map.insert(key_string, f);
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut web_resources: HashMap<String, String> = HashMap::new();

    load_web_resources(&mut web_resources, "web", &"web".to_string());

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
    
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let worker_index:usize = rng.gen_range(0..NUM_WORKERS);
                tx_channels[worker_index].send(stream).unwrap();
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_loading() {
        let mut web_resources: HashMap<String, String> = HashMap::new();
        load_web_resources(& mut web_resources, "web", &"web".to_string());
        assert!(web_resources.contains_key("/test/index.html"));
        let t = web_resources.get("/test/index.html").unwrap();
        assert!(t.contains("TEST"));
    }
}