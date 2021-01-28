use std::net::{TcpStream, TcpListener};
use std::io::Read;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    for bytes in buffer.iter() {
        println!("{}", val);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => { handle_client(stream); }
            Err(e) => { println!(" Connection error"); }
        }
    }
}
