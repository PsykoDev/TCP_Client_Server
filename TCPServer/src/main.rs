use std::collections::HashMap;
use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
    net::tcp::OwnedWriteHalf
};

enum ServerEvent {
    NewUser(OwnedWriteHalf, String),
    Message {
        username: String,
        message: String,
    },
    Disconnect(String),
}

async fn broadcast_thread(mut event_rx: mpsc::Receiver<ServerEvent>) {
    let mut users = HashMap::new();
    loop {
        match event_rx.recv().await {
            Some(evt) => match evt {
                ServerEvent::NewUser(user_channel, username) => {
                    users.insert(username, user_channel);
                },
                ServerEvent::Message { username, message } => {
                    let formatted = format!("{username}: {message}");
                    for (_, user) in users.iter_mut() {
                        if let Err(e) = user.write_all(formatted.as_bytes()).await {
                            eprintln!("Error writing to user: {}", e);
                        } else if let Err(e) = user.flush().await {
                            eprintln!("Error flushing user: {}", e);
                        }
                    }
                },
                ServerEvent::Disconnect(disconnected_username) => {
                    for mut disco_user in &mut users {
                        disco_user.1.write_all(format!("{} has been disconnected", disconnected_username).as_bytes()).await.expect("")
                    }

                    if let Some(mut disconnected_user) = users.remove(&disconnected_username) {
                        if let Err(e) = disconnected_user.shutdown().await {
                            eprintln!("Error shutting down disconnected user: {}", e);
                        }
                    }
                },
            },
            None => {
                eprintln!("Server was closed");
                break;
            },
        }
    }
}

async fn handle_client(event_tx: mpsc::Sender<ServerEvent>, mut reader: tokio::net::tcp::OwnedReadHalf, username: String) {
    println!("New Client: {}", username);

    let mut msg = [0; 4096];
    loop {
        let msg_len = reader.read(&mut msg).await;
        if let Ok(msg_len) = msg_len {
            if msg_len == 0 {
                eprintln!("Client {} disconnected.", username);
                event_tx.send(ServerEvent::Disconnect(username.clone())).await.expect("send disconnect event");
                break;
            }

            let message = String::from_utf8_lossy(&msg[..msg_len])
                .parse::<String>()
                .unwrap()
                .trim()
                .to_string();

            println!("{}: {}", username, message);
            event_tx
                .send(ServerEvent::Message {
                    username: username.clone(),
                    message,
                })
                .await
                .expect("send message to broadcast thread");
        } else {
            eprintln!("Error reading message from client {}.", username);
            break;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:1234").await?;

    let (event_tx, event_rx) = mpsc::channel(16);
    tokio::spawn(broadcast_thread(event_rx));
    println!("Waiting after new client");
    loop {
        let (socket, _) = listener.accept().await.expect("Good TcpListener, good boy <3");
        let (mut reader, writer) = socket.into_split();

        let mut nickname = [0; 20];
        let nick_len = reader.read(&mut nickname).await;

        let username = if let Ok(nick_len) = nick_len {
            String::from_utf8_lossy(&nickname[..nick_len])
                .parse::<String>()
                .expect("pseudo dead")
                .trim()
                .to_string()
        } else {
            eprintln!("Client disconnected while reading username.");
            continue;
        };

        event_tx.send(ServerEvent::NewUser(writer, username.clone())).await?;
        tokio::spawn(handle_client(event_tx.clone(), reader, username.clone()));
    }
}
