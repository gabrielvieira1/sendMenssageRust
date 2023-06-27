use std::io::Write;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Falha ao conectar ao servidor");

    let message = "Hello from client!";
    stream.write_all(message.as_bytes()).expect("Falha ao enviar a mensagem");

    drop(stream); // Fechar a conexão
}
