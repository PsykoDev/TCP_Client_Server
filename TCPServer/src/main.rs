use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let client_clone = stream.try_clone().expect("Failed to clone client stream.");
    clients.lock().unwrap().push(client_clone);

    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer);
        let bytes_unwrap = bytes_read.unwrap();
        if bytes_unwrap != 0 {
            let message = String::from_utf8_lossy(&buffer[0..bytes_unwrap]);
            println!("Received: {}", message);

            let clients = clients.lock().unwrap();
            for mut client in clients.iter() {
                client.write_all(message.as_bytes()).expect("Failed to send message to a client.");
            }
        } else {
            continue;
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").expect("Failed to bind to address.");
    println!("Server listening on 127.0.0.1:1234...");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(client) => {
                let clients_clone = Arc::clone(&clients);

                thread::spawn(move || {
                    handle_client(client, clients_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting client: {}", e);
            }
        }
    }
}
