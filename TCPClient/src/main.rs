use std::net::TcpStream;
use std::io::{Write, Read, stdin};

fn get_entry() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("read_line error");
    Ok(buf.replace("\n", "").replace("\r", ""))
}

fn exchange_with_server(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let mut buf = [0; 3];

    println!("Enter 'quit' when you want to leave");
    loop {
        write!(io, "> ")?;
        // pour afficher de suite
        io.flush()?;
        match &*get_entry()? {
            "quit" => {
                println!("bye !");
                break Ok(());
            }
            "hello" =>{
                println!(r#"Hellow \o/ "#);
            }
            line => {
                write!(stream, "{}\n", line)?;
                let received = stream.read(&mut buf)?;
                if received < 1 {
                    println!("Perte de la connexion avec le serveur");
                    break Ok(());
                }
                println!("Réponse du serveur : {:?}", String::from_utf8_lossy(&buf[..received]));
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    println!("Tentative de connexion au serveur...");
    let stream = TcpStream::connect("127.0.0.1:1234")?;
    println!("Connexion au serveur réussie !");
    exchange_with_server(stream)?;
    Ok(())
}