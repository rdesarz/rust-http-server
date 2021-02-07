use crate::server::Connection;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{io, thread};

pub struct TcpServerConnection {
    pub listener: TcpListener,
}

impl TcpServerConnection {
    pub fn new(socket: SocketAddr) -> io::Result<TcpServerConnection> {
        let listener = TcpListener::bind(socket)?;
        Ok(TcpServerConnection { listener })
    }
}

impl TcpServerConnection {
    fn handle_incoming_connection<T: Fn(&str) -> Result<String, std::io::Error>>(
        request_handler_callback: T,
        stream: &mut TcpStream,
    ) {
        let mut input_buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut input_buffer).unwrap();
        match (request_handler_callback)(std::str::from_utf8(&input_buffer).unwrap()) {
            Ok(message) => {
                if let Ok(_written_size) = stream.write(message.as_bytes()) {
                    stream.flush();
                }
            }
            Err(e) => {
                println!("Error when handling request: {:?}", e);
            }
        }
    }
}

impl Connection for TcpServerConnection {
    fn listen<T: Fn(&str) -> Result<String, std::io::Error>>(&self, request_handler_callback: T) {
        for connection in self.listener.incoming() {
            match connection {
                Ok(mut socket) => {
                    Self::handle_incoming_connection(&request_handler_callback, &mut socket);
                }
                Err(e) => println!("Error when getting client: {:?}", e),
            }
        }
    }
}
