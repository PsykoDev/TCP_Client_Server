use std::error::Error;
use tokio::{task, net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}, io};
use tokio::io::AsyncBufReadExt;

fn is_valid_utf8(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii() || c.is_ascii_alphabetic() || c.is_ascii_digit() || c.is_ascii_punctuation())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let stream = TcpStream::connect("127.0.0.1:1234").await.expect("Failed to connect to the server");
    let (mut reader, mut writer) = stream.into_split();

    println!("Bienvenue sur le client de chat multicanal!");
    println!("Pseudo: ");
    let mut nickname = String::new();
    let stdin = io::stdin(); // tokio io
    let nick = std::io::stdin();
    nick.read_line(&mut nickname).expect("");
    writer.write_all(nickname.as_bytes()).await?;


    let client_task = task::spawn(async move {

        let read_task = task::spawn(async move {
            let mut msg = [0; 4096];
            loop {
                match reader.read(&mut msg[..]).await {
                    Ok(0) => break, // Connexion fermée
                    Ok(bytes) => {
                        println!("{}", String::from_utf8_lossy(&msg[..bytes]));
                    }
                    Err(e) => {
                        eprintln!("Erreur de lecture : {}", e);
                        break;
                    }
                }
            }
        });

        let send_task = task::spawn(async move {
            let mut message = String::new();
            loop {
                std::io::stdin().read_line(&mut message).expect("");
                //let message = message.trim();
                if message == "quit" {
                    break;
                }
                if !message.is_empty(){
                    writer.write(message.as_bytes()).await.expect("Failed to write to server");
                    writer.flush().await.expect("Failed to flush");
                }
                message.clear();
            }
        });

        tokio::select! {
            _ = read_task => (),
            _ = send_task => (),
        }
    });

    client_task.await.expect("Client task failed");

    Ok(())
}
