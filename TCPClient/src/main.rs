use std::net::TcpStream;
use std::io::{Write, Read};
use std::thread;
use std::io;

fn main() -> Result<(), io::Error> {
    println!("Bienvenue sur le client de chat multicanal!");
    println!("Entrez votre nom d'utilisateur : ");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    let username = username.trim();

    let mut stream = TcpStream::connect("127.0.0.1:1234")?;
    let mut cloned_stream = stream.try_clone()?;

    // Crée un thread pour lire les messages du serveur
    let receive_thread = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = cloned_stream.read(&mut buffer);
            if bytes_read.is_ok() {
                let message = String::from_utf8_lossy(&buffer[0..bytes_read.unwrap()]);
                println!("{}", message);
            } else {
                break;
            }
        }
    });

    // Envoie les messages au serveur
    loop {
        let mut message = String::new();
        io::stdin().read_line(&mut message)?;

        let message = message.trim();
        if message == "quit" {
            break;
        }

        let full_message = format!("{}: {}\n", username, message);
        stream.write(full_message.as_bytes())?;
    }

    receive_thread.join().expect("Erreur lors de la jointure du thread de réception.");
    Ok(())
}
