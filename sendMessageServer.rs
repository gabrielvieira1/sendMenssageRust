use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Falha ao vincular o socket");

    for stream in listener.incoming() {
        let mut buffer = [0u8; 1024];
        let mut stream = stream.expect("Falha ao aceitar conexão");
        let bytes_read = stream.read(&mut buffer).expect("Falha ao ler dados do cliente");
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received message: {}", message);
    }
}
